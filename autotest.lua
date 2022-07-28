local bufnr = 9

function cargo_test ()
    vim.api.nvim_buf_set_lines(bufnr, 0, -1, false, { "=====TESTS=====" })
    vim.fn.jobstart({"cargo", "test"}, {
        stdout_buffered = true,
        on_stdout = function(_, data)
            if data then
                vim.api.nvim_buf_set_lines(bufnr, -1, -1, false, data)
            end
        end,
        on_stderr = function(_, data)
            if data then
                vim.api.nvim_buf_set_lines(bufnr, -1, -1, false, data)
            end
        end,
    })
end

function cargo_run ()
    vim.api.nvim_buf_set_lines(bufnr, 0, -1, false, { "+++++RUN+++++" })
    vim.fn.jobstart({"cargo", "run", "--", "-s", ", ", "-p", "zyxwvutsrqponmlkjihgfedcba", "hello world"}, {
        stdout_buffered = true,
        on_stdout = function(_, data)
            if data then
                vim.api.nvim_buf_set_lines(bufnr, -1, -1, false, data)
            end
        end,
        on_stderr = function(_, data)
            if data then
                vim.api.nvim_buf_set_lines(bufnr, -1, -1, false, data)
            end
        end,
    })
end

vim.api.nvim_create_autocmd("BufWritePost", {
    group = vim.api.nvim_create_augroup("autotest", { clear = true }),
    pattern = "main.rs",
    callback = cargo_run,
})

