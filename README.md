# didntask
I don't like comments. I shall purge comments. I did not ask for your help!

I dislike comments. They are usually bad and useless if not counter productive.
This is a toy project meant to learn rust for AOC 2024, use it at you *own risk*.

## How to use it
We need to build it with cargo:
```
cargo build .
```
The it can be run:

```
didnt ask [directory or file path]
```
The default is to print which comments will be removed. 
If you wish to actually remove them: pass the tag `--write`.

I strongly suggest **NOT** to pass . as directory. See the reason is very simple, 
I'm lazy and I don't account for all the different files that should not be touched (like .git) nor 
do I skip over .gitingore files. This is both laziness and also I don't believe I would 
lean much more by going so deep.

## TODO list
- [x] Read the comments
- [x] Remove the comments
- [x] Add CLI functionality
- [x] Add unit tests
- [x] Add some docs with active code
- [x] Expand to Python/Ruby
- [x] Expand to Go
- [x] Expand to SQL
- [x] Add directory support
- [x] Add recursive directory support
