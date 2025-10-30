#!/usr/bin/env perl
use strict;
use warnings;
use File::Temp qw(tempdir);
use File::Spec;
use File::Copy;
use Cwd qw(abs_path getcwd);
use Time::HiRes qw(time);

# Configuration
my $num_images = $ARGV[0] || 50;
my $script_dir = abs_path($0);
$script_dir =~ s{/[^/]+$}{};  # Remove script name to get directory
my $project_root = abs_path("$script_dir/..");

print "=== Mirror Image Performance Test ===\n";
print "Testing with $num_images images\n\n";

# Create temporary directory (auto-cleanup on exit)
my $tmpdir = tempdir(CLEANUP => 1);
print "Test directory: $tmpdir\n";

# Find logo.png in project
my $logo_path = "$project_root/logo.png";
unless (-f $logo_path) {
    die "ERROR: Cannot find logo.png at $logo_path\n";
}

print "Source image: $logo_path\n";
my $logo_size = -s $logo_path;
printf "Image size: %.2f KB\n\n", $logo_size / 1024;

# Create symlinks to logo.png
print "Creating $num_images symlinks...\n";
for my $i (1..$num_images) {
    my $link = "$tmpdir/image_$i.png";
    symlink($logo_path, $link) or die "Failed to symlink: $!\n";
}

# Generate markdown document
print "Generating markdown document...\n";
my $markdown_path = "$tmpdir/test.md";
open my $fh, '>', $markdown_path or die "Cannot write $markdown_path: $!\n";

print $fh "# Image Performance Test\n\n";
print $fh "This document contains $num_images images for performance testing.\n\n";

for my $i (1..$num_images) {
    # Mix of regular and centered images
    if ($i % 3 == 0) {
        print $fh "<p align=\"center\">\n";
        print $fh "<img src=\"image_$i.png\" width=\"400\">\n";
        print $fh "</p>\n\n";
    } else {
        print $fh "![Image $i](image_$i.png)\n\n";
    }

    print $fh "Caption for image $i\n\n";
}

close $fh;

my $doc_size = -s $markdown_path;
printf "Generated document: %.2f KB\n\n", $doc_size / 1024;

# Build mirror if needed
print "Checking mirror binary...\n";
my $mirror_bin = "$project_root/target/debug/mirror";
unless (-x $mirror_bin) {
    print "Building mirror...\n";
    chdir $project_root or die "Cannot chdir to $project_root: $!\n";
    system("cargo build") == 0 or die "Build failed\n";
}
print "Mirror binary: $mirror_bin\n\n";

# Run mirror with timing
print "=" x 60 . "\n";
print "Launching mirror (close window to complete test)...\n";
print "=" x 60 . "\n\n";

my $start_time = time();
my $exit_code = system($mirror_bin, $markdown_path, "--out-dir", $tmpdir);
my $elapsed = time() - $start_time;

print "\n" . "=" x 60 . "\n";
print "Performance Results\n";
print "=" x 60 . "\n";

printf "Total time: %.2f seconds\n", $elapsed;
printf "Images: %d\n", $num_images;
printf "Exit code: %d\n", $exit_code >> 8;

if (-f "$tmpdir/test.review.1") {
    print "Review file created: test.review.1\n";
} elsif (-f "$tmpdir/test.lgtm.1") {
    print "Approval file created: test.lgtm.1\n";
} else {
    print "No review/approval file created (window closed)\n";
}

print "\n";
print "Temporary directory will be cleaned up automatically.\n";
print "Test complete!\n";

exit 0;
