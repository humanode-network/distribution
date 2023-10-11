export const selfHostedRunners = {
  macosAarch64: ["self-hosted", "macOS", "aarch64"],
} as const;

export type RunnerOS =
  | "ubuntu-20.04"
  | "windows-latest"
  | "macos-latest"
  | (typeof selfHostedRunners)[keyof typeof selfHostedRunners];

export type Platform = {
  name: string;
  os: RunnerOS;
  buildEnvScript: string;
  essential: boolean;
  env: Record<string, string>;
  cacheKey: string;
  isBroken: boolean;
  buildTarget: string | null;
  buildTargetDir: string | null;
  extraTargetsToInstall: string[];
};

export type Platforms = Record<string, Platform>;

// An utility to apply common build script paths.
const buildEnvScriptPath = (script: string) =>
  `.github/scripts/build_env/${script}`;

// All the platforms that we support, and their respective settings.
export const all = {
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
} satisfies Platforms;

// A platform for running things that are platform-independent.
export const core = all.ubuntu satisfies Platform;
