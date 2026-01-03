from lsp_recorder import NeovimRecorder

with NeovimRecorder("test.vrl", "diag_unused").record() as recorder:
    (recorder
     .input("ggO")
     .type("unused = 5")
     .escape()
     .sleep(1)
     .type("o.output = unused")
     .escape()
     .sleep(1)
     .quit()
     )
