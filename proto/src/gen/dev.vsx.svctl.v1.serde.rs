// @generated
impl serde::Serialize for ClusterConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dev.vsx.svctl.v1.ClusterConfig", len)?;
        if self.name != 0 {
            let v = ClusterName::try_from(self.name)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.name)))?;
            struct_ser.serialize_field("name", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClusterConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClusterConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dev.vsx.svctl.v1.ClusterConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClusterConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value::<ClusterName>()? as i32);
                        }
                    }
                }
                Ok(ClusterConfig {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dev.vsx.svctl.v1.ClusterConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClusterName {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "CLUSTER_NAME_UNSPECIFIED",
            Self::Devnet => "CLUSTER_NAME_DEVNET",
            Self::Testnet => "CLUSTER_NAME_TESTNET",
            Self::MainnetBeta => "CLUSTER_NAME_MAINNET_BETA",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ClusterName {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CLUSTER_NAME_UNSPECIFIED",
            "CLUSTER_NAME_DEVNET",
            "CLUSTER_NAME_TESTNET",
            "CLUSTER_NAME_MAINNET_BETA",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClusterName;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "CLUSTER_NAME_UNSPECIFIED" => Ok(ClusterName::Unspecified),
                    "CLUSTER_NAME_DEVNET" => Ok(ClusterName::Devnet),
                    "CLUSTER_NAME_TESTNET" => Ok(ClusterName::Testnet),
                    "CLUSTER_NAME_MAINNET_BETA" => Ok(ClusterName::MainnetBeta),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Config {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.debug {
            len += 1;
        }
        if self.dry_run {
            len += 1;
        }
        if !self.source_path.is_empty() {
            len += 1;
        }
        if !self.ledger_path.is_empty() {
            len += 1;
        }
        if !self.accounts_path.is_empty() {
            len += 1;
        }
        if !self.snapshot_path.is_empty() {
            len += 1;
        }
        if !self.clusters.is_empty() {
            len += 1;
        }
        if !self.machines.is_empty() {
            len += 1;
        }
        if self.user.is_some() {
            len += 1;
        }
        if !self.home_path.is_empty() {
            len += 1;
        }
        if !self.user_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dev.vsx.svctl.v1.Config", len)?;
        if self.debug {
            struct_ser.serialize_field("debug", &self.debug)?;
        }
        if self.dry_run {
            struct_ser.serialize_field("dryRun", &self.dry_run)?;
        }
        if !self.source_path.is_empty() {
            struct_ser.serialize_field("sourcePath", &self.source_path)?;
        }
        if !self.ledger_path.is_empty() {
            struct_ser.serialize_field("ledgerPath", &self.ledger_path)?;
        }
        if !self.accounts_path.is_empty() {
            struct_ser.serialize_field("accountsPath", &self.accounts_path)?;
        }
        if !self.snapshot_path.is_empty() {
            struct_ser.serialize_field("snapshotPath", &self.snapshot_path)?;
        }
        if !self.clusters.is_empty() {
            struct_ser.serialize_field("clusters", &self.clusters)?;
        }
        if !self.machines.is_empty() {
            struct_ser.serialize_field("machines", &self.machines)?;
        }
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        if !self.home_path.is_empty() {
            struct_ser.serialize_field("homePath", &self.home_path)?;
        }
        if !self.user_name.is_empty() {
            struct_ser.serialize_field("userName", &self.user_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Config {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "debug",
            "dry_run",
            "dryRun",
            "source_path",
            "sourcePath",
            "ledger_path",
            "ledgerPath",
            "accounts_path",
            "accountsPath",
            "snapshot_path",
            "snapshotPath",
            "clusters",
            "machines",
            "user",
            "home_path",
            "homePath",
            "user_name",
            "userName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Debug,
            DryRun,
            SourcePath,
            LedgerPath,
            AccountsPath,
            SnapshotPath,
            Clusters,
            Machines,
            User,
            HomePath,
            UserName,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "debug" => Ok(GeneratedField::Debug),
                            "dryRun" | "dry_run" => Ok(GeneratedField::DryRun),
                            "sourcePath" | "source_path" => Ok(GeneratedField::SourcePath),
                            "ledgerPath" | "ledger_path" => Ok(GeneratedField::LedgerPath),
                            "accountsPath" | "accounts_path" => Ok(GeneratedField::AccountsPath),
                            "snapshotPath" | "snapshot_path" => Ok(GeneratedField::SnapshotPath),
                            "clusters" => Ok(GeneratedField::Clusters),
                            "machines" => Ok(GeneratedField::Machines),
                            "user" => Ok(GeneratedField::User),
                            "homePath" | "home_path" => Ok(GeneratedField::HomePath),
                            "userName" | "user_name" => Ok(GeneratedField::UserName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Config;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dev.vsx.svctl.v1.Config")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Config, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut debug__ = None;
                let mut dry_run__ = None;
                let mut source_path__ = None;
                let mut ledger_path__ = None;
                let mut accounts_path__ = None;
                let mut snapshot_path__ = None;
                let mut clusters__ = None;
                let mut machines__ = None;
                let mut user__ = None;
                let mut home_path__ = None;
                let mut user_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Debug => {
                            if debug__.is_some() {
                                return Err(serde::de::Error::duplicate_field("debug"));
                            }
                            debug__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DryRun => {
                            if dry_run__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dryRun"));
                            }
                            dry_run__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SourcePath => {
                            if source_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourcePath"));
                            }
                            source_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LedgerPath => {
                            if ledger_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ledgerPath"));
                            }
                            ledger_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AccountsPath => {
                            if accounts_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accountsPath"));
                            }
                            accounts_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SnapshotPath => {
                            if snapshot_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("snapshotPath"));
                            }
                            snapshot_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Clusters => {
                            if clusters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusters"));
                            }
                            clusters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Machines => {
                            if machines__.is_some() {
                                return Err(serde::de::Error::duplicate_field("machines"));
                            }
                            machines__ = Some(map_.next_value()?);
                        }
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map_.next_value()?;
                        }
                        GeneratedField::HomePath => {
                            if home_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("homePath"));
                            }
                            home_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserName => {
                            if user_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userName"));
                            }
                            user_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Config {
                    debug: debug__.unwrap_or_default(),
                    dry_run: dry_run__.unwrap_or_default(),
                    source_path: source_path__.unwrap_or_default(),
                    ledger_path: ledger_path__.unwrap_or_default(),
                    accounts_path: accounts_path__.unwrap_or_default(),
                    snapshot_path: snapshot_path__.unwrap_or_default(),
                    clusters: clusters__.unwrap_or_default(),
                    machines: machines__.unwrap_or_default(),
                    user: user__,
                    home_path: home_path__.unwrap_or_default(),
                    user_name: user_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dev.vsx.svctl.v1.Config", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExecConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.command.is_empty() {
            len += 1;
        }
        if !self.args.is_empty() {
            len += 1;
        }
        if !self.env.is_empty() {
            len += 1;
        }
        if self.dry_run {
            len += 1;
        }
        if self.color {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dev.vsx.svctl.v1.ExecConfig", len)?;
        if !self.command.is_empty() {
            struct_ser.serialize_field("command", &self.command)?;
        }
        if !self.args.is_empty() {
            struct_ser.serialize_field("args", &self.args)?;
        }
        if !self.env.is_empty() {
            struct_ser.serialize_field("env", &self.env)?;
        }
        if self.dry_run {
            struct_ser.serialize_field("dryRun", &self.dry_run)?;
        }
        if self.color {
            struct_ser.serialize_field("color", &self.color)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExecConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "command",
            "args",
            "env",
            "dry_run",
            "dryRun",
            "color",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Command,
            Args,
            Env,
            DryRun,
            Color,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "command" => Ok(GeneratedField::Command),
                            "args" => Ok(GeneratedField::Args),
                            "env" => Ok(GeneratedField::Env),
                            "dryRun" | "dry_run" => Ok(GeneratedField::DryRun),
                            "color" => Ok(GeneratedField::Color),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExecConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dev.vsx.svctl.v1.ExecConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExecConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut command__ = None;
                let mut args__ = None;
                let mut env__ = None;
                let mut dry_run__ = None;
                let mut color__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Command => {
                            if command__.is_some() {
                                return Err(serde::de::Error::duplicate_field("command"));
                            }
                            command__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Args => {
                            if args__.is_some() {
                                return Err(serde::de::Error::duplicate_field("args"));
                            }
                            args__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Env => {
                            if env__.is_some() {
                                return Err(serde::de::Error::duplicate_field("env"));
                            }
                            env__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::DryRun => {
                            if dry_run__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dryRun"));
                            }
                            dry_run__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Color => {
                            if color__.is_some() {
                                return Err(serde::de::Error::duplicate_field("color"));
                            }
                            color__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ExecConfig {
                    command: command__.unwrap_or_default(),
                    args: args__.unwrap_or_default(),
                    env: env__.unwrap_or_default(),
                    dry_run: dry_run__.unwrap_or_default(),
                    color: color__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dev.vsx.svctl.v1.ExecConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for File {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.path.is_empty() {
            len += 1;
        }
        if !self.content.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dev.vsx.svctl.v1.File", len)?;
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if !self.content.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("content", pbjson::private::base64::encode(&self.content).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for File {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path",
            "content",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
            Content,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "path" => Ok(GeneratedField::Path),
                            "content" => Ok(GeneratedField::Content),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = File;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dev.vsx.svctl.v1.File")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<File, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path__ = None;
                let mut content__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Content => {
                            if content__.is_some() {
                                return Err(serde::de::Error::duplicate_field("content"));
                            }
                            content__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(File {
                    path: path__.unwrap_or_default(),
                    content: content__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dev.vsx.svctl.v1.File", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MachineConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.ip_address.is_empty() {
            len += 1;
        }
        if !self.username.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dev.vsx.svctl.v1.MachineConfig", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.ip_address.is_empty() {
            struct_ser.serialize_field("ipAddress", &self.ip_address)?;
        }
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MachineConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "ip_address",
            "ipAddress",
            "username",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            IpAddress,
            Username,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "ipAddress" | "ip_address" => Ok(GeneratedField::IpAddress),
                            "username" => Ok(GeneratedField::Username),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MachineConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dev.vsx.svctl.v1.MachineConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MachineConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut ip_address__ = None;
                let mut username__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IpAddress => {
                            if ip_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipAddress"));
                            }
                            ip_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MachineConfig {
                    name: name__.unwrap_or_default(),
                    ip_address: ip_address__.unwrap_or_default(),
                    username: username__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dev.vsx.svctl.v1.MachineConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("dev.vsx.svctl.v1.UserConfig", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct dev.vsx.svctl.v1.UserConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UserConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UserConfig {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("dev.vsx.svctl.v1.UserConfig", FIELDS, GeneratedVisitor)
    }
}
