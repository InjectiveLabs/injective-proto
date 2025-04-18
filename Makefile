COSMOS_SDK_VERSION_TAG=v0.50.9-inj.5
IBC_GO_VERSION_TAG=v8.7.0-inj
WASMD_VERSION_TAG=v0.53.2-inj.2
INJECTIVE_CORE_VERSION_TAG=v1.15.0
INJECTIVE_INDEXER_VERSION_TAG=v1.15.6

# These variables are required by the csharp proto generation logic
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

GRPC_PROST_PLUGIN=$(WORK_DIR)/rust_plugins/protoc-gen-prost
GRPC_TONIC_PLUGIN=$(WORK_DIR)/rust_plugins/protoc-gen-tonic
GRPC_PROST_CRATE_PLUGIN=$(WORK_DIR)/rust_plugins/protoc-gen-prost-crate

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
	rm -Rf cosmos-sdk
	rm -Rf ibc-go
	rm -Rf cometbft
	rm -Rf wasmd
	rm -Rf injective-core
	rm -Rf injective-indexer
endef

define clean_packed
	rm -f *_protos.zip
endef

.PHONY: clean-all

clean-all:
	$(call clean_protos)
	$(call clean_generated)
	$(call clean_repos)
	$(call clean_packed)

clone-cosmos-sdk:
	git clone https://github.com/InjectiveLabs/cosmos-sdk.git -b $(COSMOS_SDK_VERSION_TAG) --depth 1 --single-branch

clone-ibc-go:
	git clone https://github.com/InjectiveLabs/ibc-go.git -b $(IBC_GO_VERSION_TAG) --depth 1 --single-branch

clone-wasmd:
	git clone https://github.com/InjectiveLabs/wasmd.git -b $(WASMD_VERSION_TAG) --depth 1 --single-branch

clone-injective-core:
	git clone https://github.com/InjectiveLabs/injective-core.git -b $(INJECTIVE_CORE_VERSION_TAG) --depth 1 --single-branch

clone-injective-indexer:
	git clone https://github.com/InjectiveLabs/injective-indexer.git -b $(INJECTIVE_INDEXER_VERSION_TAG) --depth 1 --single-branch

clone-all: clone-cosmos-sdk clone-ibc-go clone-wasmd clone-injective-core clone-injective-indexer

download-protos:
	buf export ./cosmos-sdk --output=third_party
	buf export ./ibc-go --exclude-imports --output=third_party
	buf export ./wasmd --exclude-imports --output=third_party
	buf export https://github.com/cosmos/ics23.git --exclude-imports --output=third_party
	cp -r injective-core/proto/injective proto/
	cp -r third_party/* proto/

download-indexer-protos:
	mkdir -p proto/exchange
	find ./injective-indexer/api/gen/grpc -type f -name "*.proto" -exec cp {} ./proto/exchange/ \; 

generate:
	buf generate --template buf.gen.yaml
	$(MAKE) download-protos
	$(MAKE) generate-rust
	$(MAKE) generate-csharp
	rm -Rf all_protos
	cp -r proto all_protos

generate-csharp:
	rm -rf ./csharp
	@for dir in $(shell find ./proto -path -prune -o -name '*.proto' -print0 | xargs -0 -n1 dirname | sort | uniq); do \
		mkdir -p ./csharp/$${dir}; \
		protoc \
		--proto_path=proto \
		--csharp_out=./csharp/$${dir} \
		--grpc_out=./csharp/$${dir} \
		$$(find ./$${dir} -type f -name '*.proto') \
		--plugin=protoc-gen-grpc=${GRPC_CSHARP_PLUGIN}; \
	done; \
	
generate-rust:
	rm -rf ./rust; 
	@for dir in $(shell find ./proto -path -prune -o -name '*.proto' -print0 | xargs -0 -n1 dirname | sort | uniq); do \
		mkdir -p ./rust/$${dir}; \
		protoc \
		--proto_path=proto \
		--prost_out=./rust/$${dir} \
		--tonic_out=./rust/$${dir} \
		$$(find ./$${dir} -type f -name '*.proto') \
		--plugin=protoc-gen-prost=${GRPC_PROST_PLUGIN} \
		--plugin=protoc-gen-prost-crate=${GRPC_PROST_CRATE_PLUGIN} \
		--plugin=protoc-gen-tonic=${GRPC_TONIC_PLUGIN}; \
	done; \
	export PATH=$(PATH):./rust_plugins; \
	PHOME="./rust/proto"; \
	protoc --proto_path=proto --prost-crate_out=$${PHOME} --prost-crate_opt=include_file=mod.rs --prost-crate_opt=no_features -I $$(find ./proto -name '*.proto' | grep -v "proto/exchange"); \
	perl -i -pe 's|\"([\w\.]+).rs\"|"$$1/$$1.rs"|g;' -pe 's|\.+(?=[\w+.]+\/)|/|g' rust/proto/mod.rs ; \
	touch $${PHOME}/amino/amino.rs $${PHOME}/cosmos/msg/v1/cosmos.msg.v1.rs $${PHOME}/cosmos/query/v1/cosmos.query.v1.rs $${PHOME}/gogoproto/gogoproto.rs ; \
	perl -i -pe 's|pub enum Validators|pub enum EnumValidators|g;' -pe 's|stake_authorization::Validators|stake_authorization::EnumValidators|g' "rust/proto/cosmos/staking/v1beta1/cosmos.staking.v1beta1.rs"; \
	protoc --proto_path=proto/exchange --prost-crate_out=$${PHOME}/exchange --prost-crate_opt=include_file=mod.rs --prost-crate_opt=no_features $$(find ./proto/exchange -name '*.proto'); \

pack:
	zip -r cpp_protos.zip cpp 
	zip -r csharp_protos.zip csharp
	zip -r java_protos.zip java
	zip -r python_protos.zip python 
	zip -r rust_protos.zip rust

run-full: clean-all clone-all download-indexer-protos generate pack
	$(call clean_repos)
	$(call clean_protos)
	$(call clean_generated)
