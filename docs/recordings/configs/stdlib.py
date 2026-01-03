from lsp_recorder import NeovimRecorder

select_next = "lua require('blink.cmp').select_next()"

with NeovimRecorder("test.vrl", "stdlib").record() as recorder:
    (recorder
     .input("ggO")
     .type("myvar = comm")
     .sleep(1)
     .command(select_next)
     .sleep(1)
     .command(select_next)
     .sleep(1)
     .enter()
     .quit()
     )
