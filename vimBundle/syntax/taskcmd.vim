" Vim syntax file
" Language:   taskcmd task definitions
" Filenames:  .taskcmd, *.taskcmd
" URL:        https://github.com/EngosSoftware/taskcmd
" License:    MIT
"
" Port of tmBundle/syntaxes/taskcmd.tmLanguage.json.

if exists('b:current_syntax')
  finish
endif

let s:cpo_save = &cpo
set cpo&vim

syn case match

" Inline code: `...`   (repository#code-inline)
" Only reachable from inside .note and .desc bodies, same as the grammar.
syn region taskcmdInlineCode matchgroup=taskcmdPunctuation
      \ start=/`/ end=/`/ oneline contained

" {{ ... }}   (interpolation.taskcmd)
syn match taskcmdInterpolation /{{[^}]*}}/

" The leading dot of every node.  What follows it decides the group; the
" candidates below are listed in reverse grammar order, because when several
" items match at the same position Vim gives priority to the one defined last.
syn match taskcmdDot /^\s*\./
      \ nextgroup=taskcmdNode,taskcmdVariable,taskcmdDesc,taskcmdNote

" .<name>   (node.taskcmd)
syn match taskcmdNode /\S\+/ contained

" .<NAME>   (variable.taskcmd) - upper case, underscores and hyphens
syn match taskcmdVariable /[A-Z_-]\+/ contained

" .desc <text>   (description.taskcmd) - runs until the next node
syn region taskcmdDesc matchgroup=taskcmdDescTag
      \ start=/desc\%(\s\|$\)/ end=/^\s*\./me=s-1,re=s-1
      \ contained keepend contains=taskcmdInlineCode

" .note <text>   (note.taskcmd) - runs until the next node
syn region taskcmdNote matchgroup=taskcmdNoteTag
      \ start=/note\%(\s\|$\)/ end=/^\s*\./me=s-1,re=s-1
      \ contained keepend contains=taskcmdInlineCode

" Blocks are unbounded upwards, so scan the whole (small) file.
syn sync fromstart

hi def link taskcmdNode          Keyword
hi def link taskcmdVariable      Constant
hi def link taskcmdDescTag       Keyword
hi def link taskcmdDesc          String
hi def link taskcmdNoteTag       Comment
hi def link taskcmdNote          Comment
hi def link taskcmdInterpolation Constant
hi def link taskcmdInlineCode    Number
hi def link taskcmdPunctuation   Delimiter

" The grammar leaves the leading dot unstyled; link it to Keyword if you would
" rather have `.cmd` coloured as one token.
hi def link taskcmdDot           Normal

let b:current_syntax = 'taskcmd'

let &cpo = s:cpo_save
unlet s:cpo_save
