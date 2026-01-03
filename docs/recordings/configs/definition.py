from lsp_recorder import NeovimRecorder

with NeovimRecorder("test.vrl", "definition").record() as recorder:
    (recorder
     .input("ggO")
     .type("myvar = 5\n")
     .type(".output = myvar")
     .escape()
     .sleep(1)
     .lsp_definition()
     .sleep(1)
     .quit()
     )
