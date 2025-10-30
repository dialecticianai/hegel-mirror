#!/usr/bin/env perl
use strict;
use warnings;
use File::Slurp qw(read_file write_file);

my $dry_run = 0;  # Set to 0 to actually apply changes

# Test files to refactor
my @test_files = qw(
    tests/parsing_tests.rs
    tests/ui_tests.rs
);

# Pattern to match and replace (handles multi-line with varying whitespace)
my $pattern = qr/
    let \s+ mut \s+ image_manager \s* = \s* ImageManager::new \( Path::new \( "\." \) \); \s*
    let \s+ chunks \s* = \s* parse_markdown \( markdown, \s* Path::new \( "\." \), \s* &mut \s+ image_manager \);
/x;

my $replacement = 'let chunks = parse_test_markdown(markdown);';

my $total_replacements = 0;

foreach my $file (@test_files) {
    unless (-f $file) {
        print "Skipping $file (not found)\n";
        next;
    }

    print "\n" . "=" x 60 . "\n";
    print "Processing: $file\n";
    print "=" x 60 . "\n";

    my $content = read_file($file);
    my $original_content = $content;
    my $count = 0;

    # Find all matches and show them
    while ($content =~ /$pattern/g) {
        $count++;
        my $match_start = $-[0];
        my $match_end = $+[0];
        my $matched_text = substr($content, $match_start, $match_end - $match_start);

        # Find line number
        my $before = substr($content, 0, $match_start);
        my $line_num = ($before =~ tr/\n//) + 1;

        print "\nMatch $count at line ~$line_num:\n";
        print "  OLD: " . $matched_text =~ s/\n/ /gr . "\n";
        print "  NEW: $replacement\n";
    }

    # Perform replacement
    $content = $original_content;
    $content =~ s/$pattern/$replacement/g;

    if ($count > 0) {
        print "\nTotal matches in $file: $count\n";
        $total_replacements += $count;

        if ($dry_run) {
            print "DRY RUN - No changes written\n";
        } else {
            write_file($file, $content);
            print "✅ Changes written to $file\n";
        }
    } else {
        print "No matches found in $file\n";
    }
}

print "\n" . "=" x 60 . "\n";
print "Summary\n";
print "=" x 60 . "\n";
print "Total replacements: $total_replacements\n";

if ($dry_run) {
    print "\n⚠️  DRY RUN MODE - No files were modified\n";
    print "To apply changes, edit the script and set: \$dry_run = 0;\n";
} else {
    print "\n✅ All changes applied successfully\n";
}
