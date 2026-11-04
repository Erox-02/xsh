---
title: "Xsh"
author: "Erox-02"
description: "A custom shell alt to bash and zsh but in rust "
created_at: "2026-11-02"
---

# Starting 

Today i am goin to write a custom shell in rust , becaause bash is way too old and zsh doesnt have wht i want .
i think imma gonna use stdin for now and write a real real basic version .

i was js trying to write a real basic verion but vs code said , nah i am gonna help u with an odd version lol , anyways 

done with the v0.00001 , it works , basic ls , cd and pwd works it runs really fast though , but it doesnt have new line after excuting , i am not going to fix tht at all , 

i will commit and move to v1

# v0.0.2

we will use reedline now instead of my prev stdin approach because i wd like to use the arrows tooo

okkk done , afteri forgot the wildcard _=> , and wasted time , now we have an working sheel tht can execute all basic command , oh lets try pacman if reedline supports tht,

holyy IT Supports pacman too , and it probably has history because of reedline , using external crates doesnt feel fair but why not?

ok now for v0.0.3 i think i will add pipes and then i will add something tht absolutely differtiates it with bash and zsh , wait i didnt try nvim , lemme try it

NVIM WORKSS TOO , reedline absolutely cooked this time .

ok i got it a lil messed up , but fixed it and now the legenedary irony , push frm my shell!!!

ok it basically gave a pathspec prob but thts for later not now 

# Regex colour highlightin and aliases, v0.0.3

ok so we gonna make a new file named rex.rs and a lot more , we will do a whole new file extension named ".rx" (short frm for regex) 


ok clearly separated the terminal execution workflow , now we gonna do something like a regex like design of my .rx extension , like 

```rx
# pattern   styles
^git\s+push green
```

looks fine , right? anyways lets write the parser first

 i wrote a lot of code and then i realized i can make it far faster and smaaller like i used a struct named rule , i dont even need it for now , i dont knwo why the hell did i even write it , and the rust auto syntax is way too annoying , i mean its good, but wht if i get flaged as ai?

 also i am seeing this a lot , the rust syntax highlighter always rewrites , my regex as a pattern data type BRUH BOTH ARE DIFF , who's gonna tell it to a syntax highlighter?

 bruh this highlighter is a rule based one but its guessing wht i am gonna write(because i have fns and impls thts how he know) but it looks lowk pretty advanced

 ok done , also made a xsh.toml , it will push aliases to my rex and new .rx 

 ok did all and made first alias but lol alot and tons of err , i cant js fix it , still gonna try until i cant ,

 bruh seriously , i js fixed a tuple syntax and got frm 12 errs to 21 :skull: 

 OHHH fixed all with a lil of way too much suffering , now i js needa solve the 2 errs in conf.rs and done i will go to sleep 

 ok so i fixed them all errs , js to not get the feature i built :sadage: , it js igonores my aliases i put on xsh.toml , i put a good enough logic , wait i think i see wht happended lemme try again
 yeah i got it head on 
 I PUT COMMAND INSTEAD OF FINal COMMAND , in Command::new() so i was myself the reason of my own suffering :sob:

 ok done it works 
 ![](assets/alias.png)

 imma gonna test the highlight regex later because i am so sleepy 

---

ok i am back , lets add a regex pattern and test it, btw its pretty easy to text xsh inside xsh and write xsh inside xsh lol!

ok i added an base.rx but it doesnt work :sob: back to debugging

Oh wow i forgot to add the highlighter in my run shell fn and was wondering why main doesnt prints my debug msgs :sob:

still doesnt work :sadage: , even after adding it in exe.rx

ok it works , but no highligh lemme check again

OK now i finally found the err with a really odd bug , my terminal doesnt render normal ansi frm ni-ansi because idk but thts probably the reason , so i will do fixed 256 bit colour approach
idk not even rgb s working , WTF

ok so i read the docs , did all tht i was supposed to do but still got the classic problem , depiciency version mismatch :cryy:

wait lemme check if my terminal supports it or not 

![](assets/t_is_fine.png)

nah my terminal is fine :lol:

ok enough of this shit now next is a diff one imma gonna get this nu-ansi out , okk verified , we wd use crossterm !