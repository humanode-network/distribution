export type Mode = {
  name: string;
  cargoCommand: string;
  cargoArgs: string;
  cargoCacheKey: string;
  platformIndependent?: true;
};

export type Modes = Record<string, Mode>;

export const code = {
  clippy: {
    name: "clippy",
    cargoCommand: "clippy",
    cargoArgs: "--workspace --all-targets -- -D warnings",
    cargoCacheKey: "clippy",
  },
  test: {
    name: "test",
    cargoCommand: "test",
    cargoArgs: "--workspace",
    cargoCacheKey: "test",
  },
  build: {
    name: "zigbuild",
    cargoCommand: "zigbuild",
    cargoArgs: "--workspace",
    cargoCacheKey: "zigbuild",
  },
  fmt: {
    name: "fmt",
    cargoCommand: "fmt",
    cargoArgs: "-- --check",
    platformIndependent: true,
    cargoCacheKey: "code",
  },
  docs: {
    name: "doc",
    cargoCommand: "doc",
    cargoArgs: "--workspace --document-private-items",
    platformIndependent: true,
    cargoCacheKey: "doc",
  },
} satisfies Modes;

export const build = {
  build: {
    name: "zigbuild",
    cargoCommand: "zigbuild",
    cargoArgs: "--workspace --release",
    cargoCacheKey: "release-zigbuild",
  },
} satisfies Modes;
