pub const LOOM_VERSION: &'static str = "1.3-SNAPSHOT";
pub const PLUGIN_VERSION: &'static str = "3.4-SNAPSHOT";

pub const MINECRAFT_1_16_5: MinecraftVersion = MinecraftVersion::new("1.16.5")
    .java_version(JavaVersion::Java8)
    .fabric_api_branch("1.16")
    .old_architectury_package()
    .forge_major_version("36")
    .architectury_api_version("1");

pub const MINECRAFT_1_17_1: MinecraftVersion = MinecraftVersion::new("1.17.1")
    .java_version(JavaVersion::Java9OrNewer(16))
    .fabric_api_branch("1.17")
    .forge_major_version("37")
    .architectury_api_version("2");

pub const MINECRAFT_1_18_1: MinecraftVersion = MinecraftVersion::new("1.18.1")
    .fabric_api_branch("1.18")
    .forge_major_version("39")
    .architectury_api_version("3");

pub const MINECRAFT_1_18_2: MinecraftVersion = MinecraftVersion::new("1.18.2")
    .forge_major_version("40")
    .architectury_api_version("4");

pub const MINECRAFT_1_19: MinecraftVersion = MinecraftVersion::new("1.19")
    .forge_major_version("41")
    .architectury_api_version("5");

pub const MINECRAFT_1_19_1: MinecraftVersion = MinecraftVersion::new("1.19.1")
    .forge_major_version("42")
    .architectury_api_version("6.3");

pub const MINECRAFT_1_19_2: MinecraftVersion = MinecraftVersion::new("1.19.2")
    .forge_major_version("43")
    .architectury_api_version("6");

pub const MINECRAFT_1_19_3: MinecraftVersion = MinecraftVersion::new("1.19.3")
    .forge_major_version("44")
    .architectury_api_version("7");

pub const MINECRAFT_1_19_4: MinecraftVersion = MinecraftVersion::new("1.19.4")
    .forge_major_version("45")
    .architectury_api_version("8");

pub const MINECRAFT_1_20_1: MinecraftVersion = MinecraftVersion::new("1.20.1")
    .forge_major_version("47")
    .architectury_api_version("9");

pub const MINECRAFT_1_20_2: MinecraftVersion = MinecraftVersion::new("1.20.2")
    .forge_major_version("48")
    .architectury_api_version("10");

pub const ALL_MINECRAFT_VERSIONS: &'static [MinecraftVersion] = &[
    MINECRAFT_1_20_2,
    MINECRAFT_1_20_1,
    MINECRAFT_1_19_4,
    MINECRAFT_1_19_3,
    MINECRAFT_1_19_2,
    MINECRAFT_1_19_1,
    MINECRAFT_1_19,
    MINECRAFT_1_18_2,
    MINECRAFT_1_18_1,
    MINECRAFT_1_17_1,
    MINECRAFT_1_16_5,
];

#[derive(Eq, PartialEq)]
pub enum JavaVersion {
    Java8,
    Java9OrNewer(u32),
}

impl JavaVersion {
    pub fn gradle_java_version(&self) -> String {
        match self {
            Self::Java8 => "1_8".to_owned(),
            Self::Java9OrNewer(version) => version.to_string(),
        }
    }

    pub fn java_major_version(&self) -> u32 {
        match self {
            Self::Java8 => 8,
            Self::Java9OrNewer(version) => *version,
        }
    }
}

#[derive(Eq)]
pub struct MinecraftVersion<'a> {
    pub version: &'a str,
    pub java_version: JavaVersion,
    pub architectury_package: &'a str,
    pub fabric_api_branch: &'a str,
    pub forge_major_version: &'a str,
    pub architectury_api_version: &'a str,
}

impl MinecraftVersion<'static> {
    pub const fn new(version: &'static str) -> Self {
        MinecraftVersion {
            version,
            java_version: JavaVersion::Java9OrNewer(17),
            architectury_package: "dev.architectury",
            fabric_api_branch: version,
            forge_major_version: "",
            architectury_api_version: "",
        }
    }
}

impl<'a> MinecraftVersion<'a> {
    pub const fn java_version(mut self, version: JavaVersion) -> Self {
        self.java_version = version;
        self
    }

    pub const fn old_architectury_package(mut self) -> Self {
        self.architectury_package = "me.shedaniel.architectury";
        self
    }

    pub const fn fabric_api_branch(mut self, branch: &'a str) -> Self {
        self.fabric_api_branch = branch;
        self
    }

    pub const fn forge_major_version(mut self, version: &'a str) -> Self {
        self.forge_major_version = version;
        self
    }

    pub const fn architectury_api_version(mut self, version: &'a str) -> Self {
        self.architectury_api_version = version;
        self
    }
}

impl<'a> PartialEq for MinecraftVersion<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
    }
}
