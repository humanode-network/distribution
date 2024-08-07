export type RunnerOS =
  | "ubuntu-22.04"
  | "windows-latest"
  | "macos-14"
  | "macos-14-large";

export type Platform = {
  name: string;
  os: RunnerOS;
  buildEnvScript: string;
  essential: boolean;
  env: Record<string, string>;
  cacheKey: string;
  artifactMarker: string | null;
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
  ubuntu2204: {
    name: "Ubuntu",
    os: "ubuntu-22.04",
    buildEnvScript: buildEnvScriptPath("ubuntu.sh"),
    essential: true,
    env: {},
    cacheKey: "ubuntu2204-amd64",
    artifactMarker: null,
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
    env: {},
    cacheKey: "windows-amd64",
    artifactMarker: null,
    isBroken: false,
    buildTarget: null, // native
    buildTargetDir: null, // native
    extraTargetsToInstall: [], // native
  },
  macos14amd64: {
    name: "macOS 14 (amd64)",
    os: "macos-14-large",
    buildEnvScript: buildEnvScriptPath("macos.sh"),
    essential: false,
    env: {},
    cacheKey: "macos-14-amd64",
    artifactMarker: null,
    isBroken: false,
    buildTarget: null, // native
    buildTargetDir: null, // native
    extraTargetsToInstall: [], // native
  },
  macos14aarch64: {
    name: "macOS 14 (aarch64)",
    os: "macos-14",
    buildEnvScript: buildEnvScriptPath("macos.sh"),
    essential: false,
    env: {},
    cacheKey: "macos-14-aarch64",
    artifactMarker: null,
    isBroken: false,
    buildTarget: null, // native
    buildTargetDir: null, // native
    extraTargetsToInstall: [], // native
  },
} satisfies Platforms;

// A platform for running things that are platform-independent.
export const core = all.ubuntu2204 satisfies Platform;
