use super::Extension;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortField {
    Name,
    Downloads,
    Publisher,
}

impl SortField {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "name" => Some(SortField::Name),
            "downloads" => Some(SortField::Downloads),
            "publisher" => Some(SortField::Publisher),
            _ => None,
        }
    }
    
    pub fn sort_extensions(&self, extensions: &mut Vec<Extension>, reverse: bool) {
        extensions.sort_by(|a, b| {
            let ordering = match self {
                SortField::Name => a.display_name.to_lowercase().cmp(&b.display_name.to_lowercase()),
                SortField::Downloads => b.downloads.cmp(&a.downloads), // Default descending for downloads
                SortField::Publisher => a.publisher.to_lowercase().cmp(&b.publisher.to_lowercase()),
            };
            
            if reverse {
                ordering.reverse()
            } else {
                ordering
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sort_field_from_str() {
        assert_eq!(SortField::from_str("name"), Some(SortField::Name));
        assert_eq!(SortField::from_str("NAME"), Some(SortField::Name));
        assert_eq!(SortField::from_str("downloads"), Some(SortField::Downloads));
        assert_eq!(SortField::from_str("publisher"), Some(SortField::Publisher));
        assert_eq!(SortField::from_str("invalid"), None);
    }
    
    #[test]
    fn test_sort_by_name() {
        let mut extensions = vec![
            Extension {
                id: "1".to_string(),
                name: "ext1".to_string(),
                publisher: "pub1".to_string(),
                version: "1.0".to_string(),
                display_name: "Zebra".to_string(),
                description: None,
                downloads: 100,
            },
            Extension {
                id: "2".to_string(),
                name: "ext2".to_string(),
                publisher: "pub2".to_string(),
                version: "1.0".to_string(),
                display_name: "Alpha".to_string(),
                description: None,
                downloads: 200,
            },
        ];
        
        SortField::Name.sort_extensions(&mut extensions, false);
        assert_eq!(extensions[0].display_name, "Alpha");
        assert_eq!(extensions[1].display_name, "Zebra");
        
        SortField::Name.sort_extensions(&mut extensions, true);
        assert_eq!(extensions[0].display_name, "Zebra");
        assert_eq!(extensions[1].display_name, "Alpha");
    }
    
    #[test]
    fn test_sort_by_downloads() {
        let mut extensions = vec![
            Extension {
                id: "1".to_string(),
                name: "ext1".to_string(),
                publisher: "pub1".to_string(),
                version: "1.0".to_string(),
                display_name: "A".to_string(),
                description: None,
                downloads: 100,
            },
            Extension {
                id: "2".to_string(),
                name: "ext2".to_string(),
                publisher: "pub2".to_string(),
                version: "1.0".to_string(),
                display_name: "B".to_string(),
                description: None,
                downloads: 200,
            },
        ];
        
        SortField::Downloads.sort_extensions(&mut extensions, false);
        assert_eq!(extensions[0].downloads, 200); // Higher downloads first
        assert_eq!(extensions[1].downloads, 100);
    }
}