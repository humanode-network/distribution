export type Mode = {
  name: string;
  cargoCommand: string;
  cargoArgs: string;
  cargoCacheKey: string;
  platformIndependent?: true;
  passZigbuildTarget?: true;
};

export type Modes = Record<string, Mode>;

export const code = {
  clippy: {
    name: "cargo clippy",
    cargoCommand: "clippy",
    cargoArgs: "--locked --workspace --all-targets -- -D warnings",
    cargoCacheKey: "clippy",
  },
  test: {
    name: "cargo test",
    cargoCommand: "test",
    cargoArgs: "--locked --workspace",
    cargoCacheKey: "test",
  },
  build: {
    name: "cargo zigbuild",
    cargoCommand: "zigbuild",
    cargoArgs: "--locked --workspace",
    cargoCacheKey: "zigbuild",
    passZigbuildTarget: true,
  },
  fmt: {
    name: "cargo fmt",
    cargoCommand: "fmt",
    cargoArgs: "-- --check",
    platformIndependent: true,
    cargoCacheKey: "code",
  },
  docs: {
    name: "cargo doc",
    cargoCommand: "doc",
    cargoArgs: "--locked --workspace --document-private-items",
    platformIndependent: true,
    cargoCacheKey: "doc",
  },
} satisfies Modes;

export const build = {
  build: {
    name: "cargo zigbuild",
    cargoCommand: "zigbuild",
    cargoArgs: "--locked --workspace --release",
    cargoCacheKey: "release-zigbuild",
    passZigbuildTarget: true,
  },
} satisfies Modes;
