#!/usr/bin/env perl
use strict;
use warnings;
use File::Slurp qw(read_file write_file);
use File::Basename qw(basename);
use Getopt::Long;

# Configuration
my $dry_run = 0;
my $reverse = 0;
my $refactor_name;
my $show_help = 0;

GetOptions(
    'dry-run'   => \$dry_run,
    'reverse'   => \$reverse,
    'refactor=s' => \$refactor_name,
    'help'      => \$show_help,
) or die "Invalid options\n";

if ($show_help || !$refactor_name) {
    print <<'HELP';
Usage: refactor.pl --refactor=NAME [OPTIONS]

Options:
    --refactor=NAME    Name of refactor definition to apply (required)
    --dry-run          Show changes without applying them
    --reverse          Apply reverse transformation
    --help             Show this help message

Refactor definitions live in scripts/refactors/*.pl

Example:
    # Apply refactor (dry-run first)
    ./scripts/refactor.pl --refactor=test_helpers --dry-run
    ./scripts/refactor.pl --refactor=test_helpers

    # Reverse a refactor
    ./scripts/refactor.pl --refactor=test_helpers --reverse

HELP
    exit($show_help ? 0 : 1);
}

# Load refactor definition
my $refactor_file = "scripts/refactors/${refactor_name}.pl";
unless (-f $refactor_file) {
    die "ERROR: Refactor definition not found: $refactor_file\n";
}

# Load the refactor definition (returns a hash)
my $refactor = do "./$refactor_file";
die "ERROR: Failed to load $refactor_file: $@\n" if $@;
die "ERROR: $refactor_file did not return a hashref\n" unless ref $refactor eq 'HASH';

# Validate refactor definition
my @required = qw(name description files forward_pattern forward_replacement);
foreach my $field (@required) {
    die "ERROR: Refactor missing required field: $field\n" unless exists $refactor->{$field};
}

# Select forward or reverse transformation
my ($pattern, $replacement, $direction);
if ($reverse) {
    die "ERROR: This refactor is not reversible (no reverse_pattern defined)\n"
        unless exists $refactor->{reverse_pattern};
    $pattern = $refactor->{reverse_pattern};
    $replacement = $refactor->{reverse_replacement};
    $direction = "REVERSE";
} else {
    $pattern = $refactor->{forward_pattern};
    $replacement = $refactor->{forward_replacement};
    $direction = "FORWARD";
}

# Print header
print "=" x 60 . "\n";
print "Refactor: $refactor->{name}\n";
print "Description: $refactor->{description}\n";
print "Direction: $direction\n";
print "Mode: " . ($dry_run ? "DRY RUN" : "APPLY") . "\n";
print "=" x 60 . "\n\n";

my $total_replacements = 0;

# Process each file
foreach my $file (@{$refactor->{files}}) {
    unless (-f $file) {
        print "⚠️  Skipping $file (not found)\n";
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
            print "⏭️  DRY RUN - No changes written\n";
        } else {
            write_file($file, $content);
            print "✅ Changes written to $file\n";
        }
    } else {
        print "No matches found in $file\n";
    }
}

# Print summary
print "\n" . "=" x 60 . "\n";
print "Summary\n";
print "=" x 60 . "\n";
print "Total replacements: $total_replacements\n";

if ($dry_run) {
    print "\n⚠️  DRY RUN MODE - No files were modified\n";
    print "To apply changes, remove --dry-run flag\n";
} else {
    print "\n✅ All changes applied successfully\n";
}

exit 0;
