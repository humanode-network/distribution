// An utility to apply common build script paths.
const buildEnvScriptPath = (script) => `.github/scripts/build_env/${script}`;

// All the platforms that we support, and their respective settings.
const allPlatforms = {
  ubuntu: {
    name: "Ubuntu",
    os: "ubuntu-20.04",
    buildEnvScript: buildEnvScriptPath("ubuntu.sh"),
    essential: true,
    env: {},
    cacheKey: "ubuntu-amd64",
    isBroken: false,
    buildTarget: "x86_64-unknown-linux-gnu.2.17",
    buildTargetDir: "x86_64-unknown-linux-gnu",
    extraTargetsToInstall: [], // native
  },
  windows: {
    name: "Windows",
    os: "windows-latest",
    buildEnvScript: buildEnvScriptPath("windows.sh"),
    essential: false,
    env: {
      CARGO_INCREMENTAL: "0",
    },
    cacheKey: "windows-amd64",
    isBroken: false,
    buildTarget: null, // native
    buildTargetDir: null, // native
    extraTargetsToInstall: [], // native
  },
  macos: {
    name: "macOS (amd64)",
    os: "macos-latest",
    buildEnvScript: buildEnvScriptPath("macos.sh"),
    essential: false,
    env: {},
    cacheKey: "macos-amd64",
    isBroken: false,
    buildTarget: null, // native
    buildTargetDir: null, // native
    extraTargetsToInstall: [], // native
  },
  macos_aarch64: {
    name: "macOS (aarch64)",
    os: ["self-hosted", "macOS", "aarch64"],
    buildEnvScript: buildEnvScriptPath("macos.sh"),
    essential: false,
    env: {},
    cacheKey: "macos-aarch64",
    isBroken: false,
    buildTarget: null, // native
    buildTargetDir: null, // native
    extraTargetsToInstall: [], // native
  },
};

const codeModes = {
  clippy: {
    name: "clippy",
    cargoCommand: "clippy",
    cargoArgs: "--all-targets -- -D warnings",
    cargoCacheKey: "clippy",
  },
  test: {
    name: "test",
    cargoCommand: "test",
    cargoCacheKey: "test",
  },
  zigbuild: {
    name: "zigbuild",
    cargoCommand: "zigbuild",
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
};

const buildModes = {
  build: {
    name: "zigbuild",
    cargoCommand: "zigbuild",
    cargoArgs: "--release",
    cargoCacheKey: "release-zigbuild",
  },
};

const code = () => {
  // Compute the effective list of platforms to use.
  const effectivePlatforms = Object.values(allPlatforms).filter(
    (platform) => !platform.isBroken && platform.essential
  );

  // Compute the effective list of modes that should run for each of the platforms.
  const effectiveModes = Object.values(codeModes).filter(
    (mode) => !mode.platformIndependent
  );

  // Compute the effective list of modes that are platform indepedent and only
  // have to be run once.
  const effectiveIndepModes = Object.values(codeModes).filter(
    (mode) => mode.platformIndependent
  );

  // Compute the individual mixins for indep modes.
  const effectiveIncludes = effectiveIndepModes.map((mode) => ({
    // Run the platform independent tests on Ubuntu.
    platform: allPlatforms.ubuntu,
    mode,
  }));

  // Prepare the effective matrix.
  const matrix = provideMatrix(
    {
      platform: effectivePlatforms,
      mode: effectiveModes,
    },
    effectiveIncludes
  );

  // Print the matrix, useful for local debugging.
  logMatrix(matrix);

  // Export the matrix so it's available to the Github Actions script.
  return matrix;
};

const build = () => {
  // Compute the effective list of platforms to use.
  const effectivePlatforms = Object.values(allPlatforms).filter(
    (platform) => !platform.isBroken
  );

  // Compute the effective list of modes that should run for each of the platforms.
  const effectiveModes = Object.values(buildModes);

  // Prepare the effective matrix.
  const matrix = provideMatrix(
    {
      platform: effectivePlatforms,
      mode: effectiveModes,
    },
    []
  );

  // Print the matrix, useful for local debugging.
  logMatrix(matrix);

  // Export the matrix so it's available to the Github Actions script.
  return matrix;
};

const evalMatrix = (dimensions, includes) => {
  const evalNext = (allVariants, key, values) =>
    allVariants.flatMap((variant) =>
      values.map((value) => ({ ...variant, [key]: value }))
    );
  const dimensionKeys = Object.keys(dimensions);
  const evaluated = dimensionKeys.reduce(
    (allVariants, dimensionKey) =>
      evalNext(allVariants, dimensionKey, dimensions[dimensionKey]),
    [{}]
  );
  return [...evaluated, ...includes];
};

const provideMatrix = (dimensions, includes) => ({
  plan: evalMatrix(dimensions, includes),
});

const logMatrix = (matrix) => console.log(JSON.stringify(matrix, null, "  "));

module.exports = {
  code,
  build,
};
