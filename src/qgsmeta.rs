#![macro_use]

#[derive(Debug)]
#[allow(dead_code)]
pub enum MetaEntries{
    Name,
    QgisMinimumVersion,
    QgisMaximumVersion,
    Description,
    About,
    Version,
    Author,
    Email,
    Changelog,
    Experimental,
    Deprecated,
    Tags,
    Homepage,
    Repository,
    Tracker,
    Icon,
    Category,
    DownloadUrl,
    FileName
}

pub fn metakey(entry: &MetaEntries) -> &'static str {
    match entry {
        &MetaEntries::Name               => "name",
        &MetaEntries::Version            => "version",
        &MetaEntries::QgisMinimumVersion => "qgisMinimumVersion",
        &MetaEntries::QgisMaximumVersion => "qgisMaximumVersion",
        &MetaEntries::Description        => "description",
        &MetaEntries::About              => "about",
        &MetaEntries::Author             => "author",
        &MetaEntries::Email              => "email",
        &MetaEntries::Changelog          => "changelog",
        &MetaEntries::Experimental       => "experimental",
        &MetaEntries::Deprecated         => "deprecated",
        &MetaEntries::Tags               => "tags",
        &MetaEntries::Homepage           => "homepage",
        &MetaEntries::Repository         => "repository",
        &MetaEntries::Tracker            => "tracker",
        &MetaEntries::Icon               => "icon",
        &MetaEntries::Category           => "category",
        &_                               => ""
    }
}

pub fn xmlkey(entry: &MetaEntries) -> &'static str {
    match entry {
        &MetaEntries::QgisMinimumVersion => "qgis_minimum_version",
        &MetaEntries::QgisMaximumVersion => "qgis_maximum_version",
        &MetaEntries::Author             => "author_name",
        &MetaEntries::DownloadUrl        => "download_url",
        &MetaEntries::FileName           => "file_name",
        _ => metakey(entry)
    }
}

macro_rules! required_entries {
    () => (
        vec![
            MetaEntries::Name,
            MetaEntries::QgisMinimumVersion,
            MetaEntries::Description,
            MetaEntries::About,
            MetaEntries::Version,
            MetaEntries::Author,
            MetaEntries::Email,
            MetaEntries::Repository
        ]
    )
}

macro_rules! attr_entries {
    () => (
        vec![
            MetaEntries::Name,
            MetaEntries::Version
        ]
    )
}

macro_rules! text_entries {
    () => (
        vec![
            MetaEntries::QgisMinimumVersion,
            MetaEntries::QgisMaximumVersion,
            MetaEntries::Version,
            MetaEntries::Email,
            MetaEntries::Experimental,
            MetaEntries::Deprecated,
            MetaEntries::Category
        ]
    )
}

macro_rules! cdata_entries {
    () => (
        vec![
            MetaEntries::Description,
            MetaEntries::About,
            MetaEntries::Homepage,
            MetaEntries::Author,
            MetaEntries::Repository,
            MetaEntries::Tracker,
            MetaEntries::Tags
        ]
    )
}
