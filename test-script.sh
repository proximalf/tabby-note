cargo build
tn=./target/debug/tabby-note

# Write to test file auto generate title
echo ""
echo "Title"
$tn -w "test-note"
# Write to test file with custom title
echo ""
echo "Write"
$tn -t "test-title" -w "test"
# Import a file that is then saved to the set directory, tho this has limited use.
echo ""
echo "Import"
$tn -o "./test/test-read.md" -t "opened-file"


echo ""
echo "All tests okay"
