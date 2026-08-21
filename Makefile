COMETBFT_VERSION_TAG=v1.0.1-inj.9
COSMOS_SDK_VERSION_TAG=v0.50.14-inj.11
IBC_GO_VERSION_TAG=v8.7.0-inj.4
WASMD_VERSION_TAG=v0.53.3-inj.3
INJECTIVE_CORE_VERSION_TAG=v1.20.3
INJECTIVE_INDEXER_VERSION_TAG=v1.20.49

define clean_protos
	echo "Cleaning protos"
	rm -Rf proto
	rm -Rf third_party
endef

define clean_generated
	rm -Rf rust
	rm -Rf cpp
	rm -Rf java
	rm -Rf python
	rm -Rf csharp
endef

define clean_repos
	rm -Rf injective-core
	rm -Rf injective-indexer
endef

define clean_packed
	rm -f *_protos.zip
endef

.PHONY: \
	clean-all clean-generated clone-all clone-injective-core clone-injective-indexer \
	download-indexer-protos download-protos generate generate-api-specs normalize-generated pack run-full sync-protos

clean-all:
	$(call clean_protos)
	$(call clean_generated)
	$(call clean_repos)
	$(call clean_packed)

clone-injective-core:
	@if [ -d injective-core ]; then \
		echo "injective-core already cloned"; \
	else \
		git clone https://github.com/InjectiveLabs/injective-core.git -b $(INJECTIVE_CORE_VERSION_TAG) --depth 1 --single-branch; \
	fi

clone-injective-indexer:
	@if [ -d injective-indexer ]; then \
		echo "injective-indexer already cloned"; \
	else \
		git clone https://github.com/InjectiveLabs/injective-indexer.git -b $(INJECTIVE_INDEXER_VERSION_TAG) --depth 1 --single-branch; \
	fi

clone-all: clone-injective-core clone-injective-indexer

download-protos: clone-injective-core
	rm -rf proto third_party
	mkdir -p proto third_party
	buf export https://github.com/InjectiveLabs/cosmos-sdk.git#tag=$(COSMOS_SDK_VERSION_TAG) --output=third_party
	buf export https://github.com/InjectiveLabs/ibc-go.git#tag=$(IBC_GO_VERSION_TAG) --exclude-imports --output=third_party
	buf export https://github.com/InjectiveLabs/wasmd.git#tag=$(WASMD_VERSION_TAG) --exclude-imports --output=third_party
	buf export https://github.com/InjectiveLabs/cometbft.git#tag=$(COMETBFT_VERSION_TAG) --exclude-imports --output=third_party
	buf export https://github.com/cosmos/ics23.git --exclude-imports --output=third_party
	cp -r injective-core/proto/injective proto/
	cp -r third_party/* proto/
	if [ -d "proto/proto" ]; then \
		cp -r proto/proto/* proto/; \
		rm -rf proto/proto; \
	fi

download-indexer-protos: clone-injective-indexer
	mkdir -p proto/exchange
	find ./injective-indexer/api/gen/grpc -type f -name "*.proto" -exec cp {} ./proto/exchange/ \; 

sync-protos: download-protos download-indexer-protos

generate:
	$(MAKE) sync-protos
	$(call clean_generated)
	buf generate --template buf.gen.yaml --timeout 0
	$(MAKE) normalize-generated
	$(MAKE) generate-api-specs
	rm -Rf all_protos
	cp -r proto all_protos

generate-api-specs:
	python3 scripts/generate_api_specs.py

normalize-generated:
	python3 scripts/normalize_generated.py

pack:
	$(call clean_packed)
	zip -r cpp_protos.zip cpp 
	zip -r csharp_protos.zip csharp
	zip -r java_protos.zip java
	zip -r python_protos.zip python 
	zip -r rust_protos.zip rust

run-full: clean-all clone-all generate pack
	$(call clean_repos)
	$(call clean_protos)
	$(call clean_generated)
