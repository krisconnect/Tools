# Encode bootcamp solana/rust 2024 :)
First ever exposure to solana/rust! :)

### My experience

## Final project
### The idea
- The werewolf game, first as a text and turn based play to earn deduction game!
- Players enter the game which has DAO like elements, they put in some money which the winner will be able to take (either the wolf or the villagers)
- This is just a PoC, later more roles and rewards can be added
- The players will be able to talk to each other via posts and will be able to vote after taking action
- Inactive players will be kicked
### Technicalities
- The game is played in turns in a cyclical fashion, alternating state between day and night.
- During the day each player must submit a post, where they can make a case about who should be voted out. After posting their comments they either vote or abstain. Each player has two life.
- During the night the wolf attacks someone they lose a life and a character is revealed from the wolf’s public key if it has any matching values in the target’s public key. The wolf must kill three villagers to win but each attack reveals more of his identity... 
- The game is won if either the wolf is killed or only one villager and the wolf is left.
### Overview of the game
1. Five players each pay an entry fee and the werewolf is randomly selected
2. Night time the werewolf bites someone
    a, A player loses 1 life
    b, A character is revealed from the werewolves address
3. Day time the village decides
    a, Everyone leaves a comment
    b, Everyone votes
4. Repeat from point 2 until the game is over
    a, Either the wolf is voted out
    b, Or there is only one villager and the wolf is left
5. Money is distributed and payed out
    a, Funds all go to the wolf or
    b, Funds are distributed between the players



