# Refactor: Introduce parse_test_markdown helper
# Eliminates ImageManager boilerplate in test files

{
    name => 'test_helpers',

    description => 'Replace ImageManager boilerplate with parse_test_markdown() helper',

    files => [
        'tests/parsing_tests.rs',
        'tests/ui_tests.rs',
    ],

    # Forward: boilerplate → helper
    forward_pattern => qr/
        let \s+ mut \s+ image_manager \s* = \s* ImageManager::new \( Path::new \( "\." \) \); \s*
        let \s+ chunks \s* = \s* parse_markdown \( markdown, \s* Path::new \( "\." \), \s* &mut \s+ image_manager \);
    /x,

    forward_replacement => 'let chunks = parse_test_markdown(markdown);',

    # Reverse: helper → boilerplate
    reverse_pattern => qr/
        let \s+ chunks \s* = \s* parse_test_markdown \( markdown \);
    /x,

    reverse_replacement => 'let mut image_manager = ImageManager::new(Path::new("."));
    let chunks = parse_markdown(markdown, Path::new("."), &mut image_manager);',
}
