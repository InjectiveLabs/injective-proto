#!/usr/bin/env python3
"""Generate OpenAPI and AsyncAPI specs from RFQ definitions.

OpenAPI: Extracted from the indexer's generated openapi3.json (Goa output).
AsyncAPI: Parsed from the RFQ proto for streaming RPCs.

Usage:
  python3 scripts/generate_api_specs.py
"""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

WORKSPACE = Path(__file__).resolve().parent.parent
INDEXER_OPENAPI = WORKSPACE / "injective-indexer" / "api" / "gen" / "http" / "openapi3.json"
PROTO_DIR = WORKSPACE / "all_protos" / "exchange"
SPEC_DIR = WORKSPACE / "spec" / "rfq"

COMMENT_RE = re.compile(r"^\s*//\s?(.*)")
FIELD_RE = re.compile(
    r"^\s*(?P<rule>repeated\s+)?(?P<type>\S+)\s+(?P<name>\S+)\s*=\s*(?P<number>\d+)\s*;"
)
MESSAGE_RE = re.compile(r"^\s*message\s+(\w+)\s*\{")
SERVICE_RE = re.compile(r"^\s*service\s+(\w+)\s*\{")
RPC_RE = re.compile(
    r"^\s*rpc\s+(\w+)\s*\(\s*(stream\s+)?(\w+)\s*\)\s*returns\s*\(\s*(stream\s+)?(\w+)\s*\)\s*;"
)
CLOSE_RE = re.compile(r"^\s*\}")

PROTO_TYPE_MAP = {
    "string": ("string", None),
    "bool": ("boolean", None),
    "int32": ("integer", "int32"),
    "int64": ("integer", "int64"),
    "sint32": ("integer", "int32"),
    "sint64": ("integer", "int64"),
    "uint32": ("integer", "uint32"),
    "uint64": ("integer", "uint64"),
    "float": ("number", "float"),
    "double": ("number", "double"),
    "bytes": ("string", "byte"),
}


# ---------------------------------------------------------------------------
# OpenAPI: extract RFQ subset from indexer's generated spec
# ---------------------------------------------------------------------------

def find_refs(obj, refs=None):
    if refs is None:
        refs = set()
    if isinstance(obj, dict):
        for k, v in obj.items():
            if k == "$ref" and isinstance(v, str) and "/schemas/" in v:
                refs.add(v.split("/")[-1])
            find_refs(v, refs)
    elif isinstance(obj, list):
        for item in obj:
            find_refs(item, refs)
    return refs


def collect_schemas_recursive(schema_name, all_schemas, collected):
    if schema_name in collected or schema_name not in all_schemas:
        return
    collected[schema_name] = all_schemas[schema_name]
    for ref in find_refs(all_schemas[schema_name]):
        collect_schemas_recursive(ref, all_schemas, collected)


def generate_openapi() -> str:
    if not INDEXER_OPENAPI.exists():
        print(
            f"WARNING: {INDEXER_OPENAPI} not found, skipping OpenAPI generation\n"
            "  Run 'make clone-injective-indexer' first",
            file=sys.stderr,
        )
        return ""

    full_spec = json.loads(INDEXER_OPENAPI.read_text(encoding="utf-8"))

    rfq_paths = {
        path: methods
        for path, methods in full_spec.get("paths", {}).items()
        if "/rfq" in path.lower()
    }

    all_schemas = full_spec.get("components", {}).get("schemas", {})
    needed_refs = find_refs(rfq_paths)
    collected_schemas: dict = {}
    for ref in needed_refs:
        collect_schemas_recursive(ref, all_schemas, collected_schemas)

    subset = {
        "openapi": full_spec.get("openapi", "3.0.3"),
        "info": {
            "title": "TrueCurrent RFQ API",
            "version": "1.0.0",
            "description": (
                "TrueCurrent RFQ API for perpetual futures trading on Injective. "
                "Unary RPCs for settlements, conditional orders, and transaction preparation.\n"
                "See https://docs.tc.xyz for full documentation."
            ),
            "contact": {"name": "TrueCurrent", "url": "https://tc.xyz"},
            "license": {
                "name": "Apache-2.0",
                "url": "https://github.com/InjectiveLabs/injective-rfq-toolkit/blob/master/LICENSE",
            },
        },
        "servers": [
            {
                "url": "https://testnet.sentry.exchange.grpc-web.injective.network",
                "description": "Testnet RFQ API",
            },
        ],
        "paths": dict(sorted(rfq_paths.items())),
        "components": {"schemas": dict(sorted(collected_schemas.items()))},
    }

    return json.dumps(subset, indent=2, ensure_ascii=False)


# ---------------------------------------------------------------------------
# AsyncAPI: parse proto for streaming RPCs
# ---------------------------------------------------------------------------

def parse_proto_messages(path):
    lines = path.read_text(encoding="utf-8").splitlines()
    messages = {}
    pending_comment = []
    current_msg = None

    for line in lines:
        cm = COMMENT_RE.match(line)
        if cm:
            pending_comment.append(cm.group(1))
            continue

        mm = MESSAGE_RE.match(line)
        if mm:
            name = mm.group(1)
            current_msg = {"comment": " ".join(pending_comment).strip(), "fields": []}
            messages[name] = current_msg
            pending_comment.clear()
            continue

        fm = FIELD_RE.match(line)
        if fm and current_msg is not None:
            current_msg["fields"].append({
                "name": fm.group("name"),
                "type": fm.group("type"),
                "repeated": fm.group("rule") is not None,
                "comment": " ".join(pending_comment).strip(),
            })
            pending_comment.clear()
            continue

        if CLOSE_RE.match(line):
            if current_msg is not None:
                current_msg = None
            pending_comment.clear()
            continue

        if not line.strip():
            pending_comment.clear()

    return messages


def parse_proto_rpcs(path):
    lines = path.read_text(encoding="utf-8").splitlines()
    rpcs = []
    service_name = None
    pending_comment = []

    for line in lines:
        cm = COMMENT_RE.match(line)
        if cm:
            pending_comment.append(cm.group(1))
            continue

        sm = SERVICE_RE.match(line)
        if sm:
            service_name = sm.group(1)
            pending_comment.clear()
            continue

        rm = RPC_RE.match(line)
        if rm:
            rpcs.append({
                "service": service_name,
                "name": rm.group(1),
                "comment": " ".join(pending_comment).strip(),
                "client_streaming": rm.group(2) is not None,
                "input_type": rm.group(3),
                "server_streaming": rm.group(4) is not None,
                "output_type": rm.group(5),
            })
            pending_comment.clear()
            continue

        if not line.strip():
            pending_comment.clear()

    return rpcs


def field_to_schema(f, all_messages):
    t = f["type"]
    schema = {}

    if t in PROTO_TYPE_MAP:
        json_type, fmt = PROTO_TYPE_MAP[t]
        schema["type"] = json_type
        if fmt:
            schema["format"] = fmt
    elif t in all_messages:
        schema["$ref"] = f"#/components/schemas/{t}"
    else:
        schema["type"] = "string"

    if f["comment"] and "$ref" not in schema:
        schema["description"] = f["comment"]

    if f["repeated"]:
        wrapper = {"type": "array", "items": schema}
        if f["comment"]:
            wrapper["description"] = f["comment"]
        return wrapper

    return schema


def msg_to_schema(name, msg, all_messages):
    schema = {"type": "object"}
    if msg["comment"]:
        schema["description"] = msg["comment"]
    props = {}
    for f in msg["fields"]:
        props[f["name"]] = field_to_schema(f, all_messages)
    if props:
        schema["properties"] = props
    return schema


def collect_message_refs(msg_name, all_messages, collected):
    if msg_name in collected or msg_name not in all_messages:
        return
    collected.add(msg_name)
    for f in all_messages[msg_name]["fields"]:
        if f["type"] in all_messages:
            collect_message_refs(f["type"], all_messages, collected)


def generate_asyncapi() -> str:
    rfq_proto = PROTO_DIR / "injective_rfq_rpc.proto"
    if not rfq_proto.exists():
        print(f"WARNING: {rfq_proto} not found, skipping AsyncAPI generation", file=sys.stderr)
        return ""

    all_messages = parse_proto_messages(rfq_proto)
    rpcs = parse_proto_rpcs(rfq_proto)
    streaming_rpcs = [r for r in rpcs if r["client_streaming"] or r["server_streaming"]]

    needed_messages = set()
    for rpc in streaming_rpcs:
        collect_message_refs(rpc["input_type"], all_messages, needed_messages)
        collect_message_refs(rpc["output_type"], all_messages, needed_messages)

    channels = {}
    operations = {}
    component_messages = {}

    for rpc in streaming_rpcs:
        channel_id = rpc["name"][0].lower() + rpc["name"][1:]
        address = f"/{rpc['service']}/{rpc['name']}"

        if rpc["client_streaming"] and rpc["server_streaming"]:
            channels[channel_id] = {
                "address": address,
                "description": rpc["comment"],
                "messages": {
                    "send": {"$ref": f"#/components/messages/{rpc['input_type']}"},
                    "receive": {"$ref": f"#/components/messages/{rpc['output_type']}"},
                },
            }
            operations[f"send{rpc['name']}"] = {
                "action": "send",
                "channel": {"$ref": f"#/channels/{channel_id}"},
                "messages": [{"$ref": f"#/channels/{channel_id}/messages/send"}],
                "summary": f"Send message to {rpc['name']}",
            }
            operations[f"receive{rpc['name']}"] = {
                "action": "receive",
                "channel": {"$ref": f"#/channels/{channel_id}"},
                "messages": [{"$ref": f"#/channels/{channel_id}/messages/receive"}],
                "summary": f"Receive message from {rpc['name']}",
            }
        elif rpc["server_streaming"]:
            channels[channel_id] = {
                "address": address,
                "description": rpc["comment"],
                "messages": {
                    "subscribe": {"$ref": f"#/components/messages/{rpc['output_type']}"},
                },
            }
            operations[f"receive{rpc['name']}"] = {
                "action": "receive",
                "channel": {"$ref": f"#/channels/{channel_id}"},
                "messages": [{"$ref": f"#/channels/{channel_id}/messages/subscribe"}],
                "summary": rpc["comment"] or f"Receive {rpc['name']} updates",
            }

        for msg_type in (rpc["input_type"], rpc["output_type"]):
            if msg_type not in component_messages and msg_type in all_messages:
                msg = all_messages[msg_type]
                component_messages[msg_type] = {
                    "name": msg_type,
                    "title": msg["comment"] if msg["comment"] else msg_type,
                    "contentType": "application/json",
                    "payload": {"$ref": f"#/components/schemas/{msg_type}"},
                }

    schemas = {}
    for name in sorted(needed_messages):
        if name in all_messages:
            schemas[name] = msg_to_schema(name, all_messages[name], all_messages)

    spec = {
        "asyncapi": "3.0.0",
        "info": {
            "title": "TrueCurrent RFQ Streaming API",
            "version": "1.0.0",
            "description": (
                "TrueCurrent RFQ streaming API for perpetual futures trading on Injective. "
                "Bidirectional gRPC streams over WebSocket (gRPC-Web) for real-time "
                "quote discovery and settlement.\n"
                "See https://docs.tc.xyz for full documentation."
            ),
            "contact": {"name": "TrueCurrent", "url": "https://tc.xyz"},
            "license": {
                "name": "Apache-2.0",
                "url": "https://github.com/InjectiveLabs/injective-rfq-toolkit/blob/master/LICENSE",
            },
        },
        "servers": {
            "testnet": {
                "host": "testnet.rfq.ws.injective.network",
                "protocol": "wss",
                "description": "Testnet WebSocket endpoint (gRPC-Web)",
            },
        },
        "defaultContentType": "application/json",
        "channels": channels,
        "operations": operations,
        "components": {
            "messages": component_messages,
            "schemas": schemas,
        },
    }

    return json.dumps(spec, indent=2, ensure_ascii=False)


def main() -> None:
    SPEC_DIR.mkdir(parents=True, exist_ok=True)

    openapi = generate_openapi()
    if openapi:
        out = SPEC_DIR / "openapi.json"
        out.write_text(openapi + "\n", encoding="utf-8")
        print(f"Generated {out.relative_to(WORKSPACE)}")

    asyncapi = generate_asyncapi()
    if asyncapi:
        out = SPEC_DIR / "asyncapi.json"
        out.write_text(asyncapi + "\n", encoding="utf-8")
        print(f"Generated {out.relative_to(WORKSPACE)}")


if __name__ == "__main__":
    main()
