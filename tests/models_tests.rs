/// Unit tests for models layer (pure logic, no UI dependencies)

mod selection_tests {
    use mirror::models::Selection;

    #[test]
    fn test_default_selection_is_inactive() {
        let selection = Selection::default();
        assert!(!selection.is_active());
        assert_eq!(selection.start_line, None);
        assert_eq!(selection.end_line, None);
        assert!(!selection.is_dragging);
    }

    #[test]
    fn test_start_drag_sets_both_start_and_end() {
        let mut selection = Selection::default();
        selection.start_drag(5);

        assert!(selection.is_active());
        assert_eq!(selection.start_line, Some(5));
        assert_eq!(selection.end_line, Some(5));
        assert!(selection.is_dragging);
    }

    #[test]
    fn test_update_drag_changes_end_line() {
        let mut selection = Selection::default();
        selection.start_drag(5);
        selection.update_drag(10);

        assert!(selection.is_active());
        assert_eq!(selection.start_line, Some(5));
        assert_eq!(selection.end_line, Some(10));
        assert!(selection.is_dragging);
    }

    #[test]
    fn test_update_drag_only_works_while_dragging() {
        let mut selection = Selection::default();
        selection.start_drag(5);
        selection.end_drag();
        selection.update_drag(10);

        // End line should not have changed
        assert_eq!(selection.start_line, Some(5));
        assert_eq!(selection.end_line, Some(5));
        assert!(!selection.is_dragging);
    }

    #[test]
    fn test_end_drag_clears_dragging_flag() {
        let mut selection = Selection::default();
        selection.start_drag(5);
        assert!(selection.is_dragging);

        selection.end_drag();
        assert!(!selection.is_dragging);
        assert!(selection.is_active()); // Selection still exists
    }

    #[test]
    fn test_clear_resets_all_state() {
        let mut selection = Selection::default();
        selection.start_drag(5);
        selection.update_drag(10);

        selection.clear();

        assert!(!selection.is_active());
        assert_eq!(selection.start_line, None);
        assert_eq!(selection.end_line, None);
        assert!(!selection.is_dragging);
    }

    #[test]
    fn test_contains_line_forward_selection() {
        let mut selection = Selection::default();
        selection.start_drag(5);
        selection.update_drag(10);

        assert!(!selection.contains_line(4));
        assert!(selection.contains_line(5));
        assert!(selection.contains_line(7));
        assert!(selection.contains_line(10));
        assert!(!selection.contains_line(11));
    }

    #[test]
    fn test_contains_line_backward_selection() {
        let mut selection = Selection::default();
        selection.start_drag(10);
        selection.update_drag(5);

        assert!(!selection.contains_line(4));
        assert!(selection.contains_line(5));
        assert!(selection.contains_line(7));
        assert!(selection.contains_line(10));
        assert!(!selection.contains_line(11));
    }

    #[test]
    fn test_contains_line_single_line_selection() {
        let mut selection = Selection::default();
        selection.start_drag(7);

        assert!(!selection.contains_line(6));
        assert!(selection.contains_line(7));
        assert!(!selection.contains_line(8));
    }

    #[test]
    fn test_contains_line_with_no_selection() {
        let selection = Selection::default();
        assert!(!selection.contains_line(0));
        assert!(!selection.contains_line(1));
        assert!(!selection.contains_line(100));
    }
}

mod comment_tests {
    use mirror::models::Comment;

    #[test]
    fn test_comment_new_stores_all_fields() {
        let comment = Comment::new("Test comment".to_string(), 10, 5, 12, 20);

        assert_eq!(comment.text, "Test comment");
        assert_eq!(comment.line_start, 10);
        assert_eq!(comment.col_start, 5);
        assert_eq!(comment.line_end, 12);
        assert_eq!(comment.col_end, 20);
    }

    #[test]
    fn test_comment_format_single_line() {
        let comment = Comment::new("Fix typo".to_string(), 5, 10, 5, 15);

        let formatted = comment.format();
        assert_eq!(formatted, "[L5:C10 → L5:C15] Fix typo");
    }

    #[test]
    fn test_comment_format_multi_line() {
        let comment = Comment::new(
            "This section needs clarification".to_string(),
            10,
            0,
            15,
            42,
        );

        let formatted = comment.format();
        assert_eq!(
            formatted,
            "[L10:C0 → L15:C42] This section needs clarification"
        );
    }

    #[test]
    fn test_comment_format_empty_text() {
        let comment = Comment::new("".to_string(), 1, 1, 1, 1);

        let formatted = comment.format();
        assert_eq!(formatted, "[L1:C1 → L1:C1] ");
    }
}

mod layout_tests {
    use mirror::models::LayoutMap;

    #[test]
    fn test_layout_map_new_is_empty() {
        let layout_map = LayoutMap::new();
        assert_eq!(layout_map.get_line_y(0), None);
        assert_eq!(layout_map.get_line_y(1), None);
        assert_eq!(layout_map.get_line_y(100), None);
    }

    #[test]
    fn test_layout_map_record_and_get() {
        let mut layout_map = LayoutMap::new();

        // Record a chunk spanning lines 5-10 at Y positions 100.0-200.0
        layout_map.record_chunk(5, 10, 100.0, 200.0);

        // Line 5 should be at the start (100.0)
        assert_eq!(layout_map.get_line_y(5), Some(100.0));

        // Line 10 should be near the end (but not at 200.0, that's the bottom of the chunk)
        assert!(layout_map.get_line_y(10).is_some());

        // Lines outside the range should return None
        assert_eq!(layout_map.get_line_y(4), None);
        assert_eq!(layout_map.get_line_y(11), None);
    }

    #[test]
    fn test_layout_map_single_line_chunk() {
        let mut layout_map = LayoutMap::new();

        // Single line chunk (line 5 occupies Y 50.0-70.0)
        layout_map.record_chunk(5, 5, 50.0, 70.0);

        assert_eq!(layout_map.get_line_y(5), Some(50.0));
        assert_eq!(layout_map.get_line_y(4), None);
        assert_eq!(layout_map.get_line_y(6), None);
    }

    #[test]
    fn test_layout_map_clear() {
        let mut layout_map = LayoutMap::new();

        layout_map.record_chunk(1, 3, 0.0, 30.0);
        layout_map.record_chunk(4, 6, 30.0, 60.0);

        assert!(layout_map.get_line_y(1).is_some());
        assert!(layout_map.get_line_y(5).is_some());

        layout_map.clear();

        assert_eq!(layout_map.get_line_y(1), None);
        assert_eq!(layout_map.get_line_y(5), None);
    }

    #[test]
    fn test_layout_map_multiple_chunks() {
        let mut layout_map = LayoutMap::new();

        // First chunk: lines 1-3 at Y 0-30
        layout_map.record_chunk(1, 3, 0.0, 30.0);
        // Second chunk: lines 4-6 at Y 30-60
        layout_map.record_chunk(4, 6, 30.0, 60.0);

        // Check first chunk
        assert_eq!(layout_map.get_line_y(1), Some(0.0));
        assert!(layout_map.get_line_y(2).is_some());
        assert!(layout_map.get_line_y(3).is_some());

        // Check second chunk
        assert_eq!(layout_map.get_line_y(4), Some(30.0));
        assert!(layout_map.get_line_y(5).is_some());
        assert!(layout_map.get_line_y(6).is_some());

        // Outside both chunks
        assert_eq!(layout_map.get_line_y(0), None);
        assert_eq!(layout_map.get_line_y(7), None);
    }

    #[test]
    fn test_layout_map_get_y_range() {
        let mut layout_map = LayoutMap::new();

        // Lines 5-10 occupy Y 100-200
        layout_map.record_chunk(5, 10, 100.0, 200.0);

        // Get range for lines 5-7
        let range = layout_map.get_y_range(5, 7);
        assert!(range.is_some());

        let (start_y, end_y) = range.unwrap();
        assert_eq!(start_y, 100.0); // Start at line 5
        assert!(end_y > start_y); // End should be after start
        assert!(end_y < 200.0); // Should be before chunk end

        // Range outside chunk should return None
        assert_eq!(layout_map.get_y_range(1, 3), None);
    }

    #[test]
    fn test_layout_map_line_zero_ignored() {
        let mut layout_map = LayoutMap::new();

        // Line 0 is ignored (as per implementation)
        layout_map.record_chunk(0, 5, 0.0, 50.0);

        // Should have no recorded chunks
        assert_eq!(layout_map.get_line_y(0), None);
        assert_eq!(layout_map.get_line_y(1), None);
    }
}
