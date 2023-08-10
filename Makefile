# These two variables are required by the csharp proto generation logic
WORK_DIR=$(shell pwd)
OS_NAME=$(shell uname -s)
OS_ARCH=$(shell uname -p)

ifeq ($(OS_NAME), Darwin)
	GRPC_CSHARP_PLUGIN=$(WORK_DIR)/grpc.tools/tools/macosx_x64/grpc_csharp_plugin
else
	ifeq ($(OS_ARCH), arm)
		GRPC_CSHARP_PLUGIN=$(WORK_DIR)/grpc.tools/tools/linux_arm64/grpc_csharp_plugin
	else
		GRPC_CSHARP_PLUGIN=$(WORK_DIR)/grpc.tools/tools/linux_x64/grpc_csharp_plugin
	endif
endif

define clean_protos
	echo "Cleaning protos"
	rm -Rf proto
	rm -Rf third_party
endef

define clean_generated
	rm -Rf rust
	rm -Rf cpp
	rm -Rf python
	rm -Rf csharp
endef

define clean_repos
	rm -Rf cometbft
	rm -Rf injective-core
	rm -Rf injective-indexer
endef

.PHONY: clean-all

clean-all:
	$(call clean_protos)
	$(call clean_generated)
	$(call clean_repos)

clone-injective-core:
	git clone https://github.com/InjectiveLabs/injective-core.git -b master --depth 1 --single-branch

clone-injective-indexer:
	git clone https://github.com/InjectiveLabs/injective-indexer.git -b master --depth 1 --single-branch

clone-cometbft:
	git clone https://github.com/cometbft/cometbft.git -b v0.37.0 --depth 1 --single-branch

clone-all: clone-injective-core clone-injective-indexer clone-cometbft

download-protos:
	mkdir -p proto/exchange
	buf export buf.build/cosmos/cosmos-sdk:v0.47.0 --output=third_party
	buf export https://github.com/cosmos/ibc-go.git --exclude-imports --output=third_party
	buf export ./cometbft --exclude-imports --output=third_party
	buf export https://github.com/CosmWasm/wasmd.git --exclude-imports --output=third_party
	buf export https://github.com/cosmos/ics23.git --exclude-imports --output=third_party
	cp -r injective-core/proto/injective proto/
	cp -r third_party/* proto/
	find ./injective-indexer/api/gen/grpc -type f -name "*.proto" -exec cp {} ./proto/exchange/ \; 

generate: generate-csharp
	cd ./proto && buf generate --template ../buf.gen.yaml

generate-csharp:
	@for dir in $(shell find ./proto -path -prune -o -name '*.proto' -print0 | xargs -0 -n1 dirname | sort | uniq); do \
		mkdir -p ./csharp/$${dir}; \
		protoc \
		--proto_path=proto \
		--csharp_out=./csharp/$${dir} \
		--grpc_out=./csharp/$${dir} \
		$$(find ./$${dir} -type f -name '*.proto') \
		--plugin=protoc-gen-grpc=${GRPC_CSHARP_PLUGIN}; \
	done; \


run-full: clean-all clone-all download-protos generate
	$(call clean_repos)
	$(call clean_protos)
