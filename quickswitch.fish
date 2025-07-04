#!/bin/fish

function qs
    set tmp_file (mktemp)

    /home/debin/Codes/tools/quickswitch/quickswitch/target/release/quickswitch --output-file $tmp_file

    set dest_path (cat $tmp_file)

    rm $tmp_file

    if test -n "$dest_path" -a -d "$dest_path"
        cd -- $dest_path
    end
end