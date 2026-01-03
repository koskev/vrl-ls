from lsp_recorder import NeovimRecorder

with NeovimRecorder("test.vrl", "rename").record() as recorder:
    (recorder
     .input("ggO")
     .type("myvar = 5\n\n")
     .type(".output = myvar\n")
     .type(".output2 = 3 + myvar")
     .escape()
     .sleep(1)
     .lsp_rename()
     .sleep(1)
     .type("Renamed")
     .enter()
     .sleep(1)
     .quit()
     )
