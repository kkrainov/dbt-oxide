use crate::manifest::OxideMacro;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Locality {
    Core = 1,
    Imported = 2,
    Root = 3,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MacroCandidate {
    pub locality: Locality,
    pub unique_id: String,
    pub package_name: String,
    pub name: String,
}

impl MacroCandidate {
    fn new(macro_def: &OxideMacro, locality: Locality) -> Self {
        Self {
            locality,
            unique_id: macro_def.unique_id.clone(),
            package_name: macro_def.package_name.clone(),
            name: macro_def.name.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MaterializationCandidate {
    pub locality: Locality,
    pub unique_id: String,
    pub package_name: String,
    pub name: String,
    pub specificity: usize,
}

impl MaterializationCandidate {
    fn from_macro_candidate(candidate: MacroCandidate, specificity: usize) -> Self {
        Self {
            locality: candidate.locality,
            unique_id: candidate.unique_id,
            package_name: candidate.package_name,
            name: candidate.name,
            specificity,
        }
    }
}

impl PartialEq for MaterializationCandidate {
    fn eq(&self, other: &Self) -> bool {
        self.specificity == other.specificity && self.locality == other.locality
    }
}

impl Eq for MaterializationCandidate {}

impl PartialOrd for MaterializationCandidate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MaterializationCandidate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match other.specificity.cmp(&self.specificity) {
            std::cmp::Ordering::Equal => self.locality.cmp(&other.locality),
            ordering => ordering,
        }
    }
}

pub fn get_locality(
    macro_package: &str,
    root_project_name: &str,
    internal_packages: &HashSet<String>,
) -> Locality {
    if macro_package == root_project_name {
        Locality::Root
    } else if internal_packages.contains(macro_package) {
        Locality::Core
    } else {
        Locality::Imported
    }
}

pub fn get_materialization_macro_name(name: &str, adapter_type: &str) -> String {
    format!("materialization_{}_{}", name, adapter_type)
}

pub fn find_macros_by_name(
    macros: &HashMap<String, OxideMacro>,
    name: &str,
    root_project_name: &str,
    internal_packages: &HashSet<String>,
) -> Vec<MacroCandidate> {
    macros
        .values()
        .filter(|m| m.name == name)
        .map(|m| {
            let locality = get_locality(&m.package_name, root_project_name, internal_packages);
            MacroCandidate::new(m, locality)
        })
        .collect()
}

pub fn find_macro_by_name(
    macros: &HashMap<String, OxideMacro>,
    name: &str,
    root_project_name: &str,
    internal_packages: &HashSet<String>,
    package: Option<&str>,
) -> Option<String> {
    let mut candidates = find_macros_by_name(macros, name, root_project_name, internal_packages);

    if let Some(pkg) = package {
        candidates.retain(|c| c.package_name == pkg);
    }

    candidates.sort();

    candidates.last().map(|c| c.unique_id.clone())
}

pub fn find_materialization_macro_by_name(
    macros: &HashMap<String, OxideMacro>,
    project_name: &str,
    materialization_name: &str,
    adapter_types: &[String],
    internal_packages: &HashSet<String>,
    allow_package_override: bool,
) -> Option<String> {
    let mut all_candidates: Vec<MaterializationCandidate> = Vec::new();

    for (specificity, adapter_type) in adapter_types.iter().enumerate() {
        let full_name = get_materialization_macro_name(materialization_name, adapter_type);
        let macro_candidates =
            find_macros_by_name(macros, &full_name, project_name, internal_packages);

        for macro_candidate in macro_candidates {
            all_candidates.push(MaterializationCandidate::from_macro_candidate(
                macro_candidate,
                specificity,
            ));
        }
    }

    if !all_candidates.is_empty() && !allow_package_override {
        let has_core = all_candidates.iter().any(|c| c.locality == Locality::Core);

        if has_core {
            all_candidates.retain(|c| c.locality != Locality::Imported);
        }
    }

    all_candidates.sort();

    all_candidates.last().map(|c| c.unique_id.clone())
}
