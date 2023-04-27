import { GeneratedType, Registry } from "@cosmjs/proto-signing";
import { AminoTypes } from "@cosmjs/stargate";
import {
  cosmosProtoRegistry,
  cosmwasmProtoRegistry,
  ibcProtoRegistry,
  osmosisProtoRegistry,
  cosmosAminoConverters,
  cosmwasmAminoConverters,
  ibcAminoConverters,
  osmosisAminoConverters,
} from "osmojs";

const protoRegistry: ReadonlyArray<[string, GeneratedType]> = [
  ...cosmosProtoRegistry,
  ...cosmwasmProtoRegistry,
  ...ibcProtoRegistry,
  ...osmosisProtoRegistry,
];

const aminoConverters = {
  ...cosmosAminoConverters,
  ...cosmwasmAminoConverters,
  ...ibcAminoConverters,
  ...osmosisAminoConverters,
};

const registry = new Registry(protoRegistry);
const aminoTypes = new AminoTypes(aminoConverters);

export { registry, aminoTypes };
