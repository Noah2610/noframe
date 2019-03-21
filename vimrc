let g:rust_recommended_style = 0

autocmd BufNewFile,BufRead *.rs nmap <buffer> <C-s> :w<CR>

nmap <leader>r :!cargo build<CR>
nmap <leader>t :!cargo test<CR>
