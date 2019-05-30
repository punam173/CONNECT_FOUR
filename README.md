# CONNECT_FOUR
Project’s Vision and Execution Plan • Since, it is an interactive game where both the player needs to play there move. I will be implementing a command line interface which will ask both the user for their turn. • Most usual size is 7column and 6 rows, but the size could vary. I will have a random size in each game. It would be appeared as a matrix in command line with the rows and column numbers displayed. • Option for each player would be to choose a column. Once a column is chosen by a player, a new matrix will appear with the selected column being marked from the bottom. • Each player’s option will be marked different. • Methods will be implemented to check the alignment of four horizontally, vertically and diagonally. • If four marks are aligned horizontally, vertically and diagonally then the game will be stopped and the winner among the two players will be announced. • The game will continue until a winner is found or no position in the column is left to choose by the players.
This game is implemented using Rust language. this game has been developed as a part of project under the course The Rust Programming Language.

Steps to Run the Project:

1. Install Rust on your computer 

   to run rust type the following command in terminal.
   
      curl https://sh.rustup.rs -sSf | sh

2. If rust is installed, you can check for the version using command below :
   
      rustc --version
    
      if it shows the version , it means rust is installed on the computer.
      
3. Clone the git repository https://github.com/krahul84/ConnectFour

4. Unzip the folder downloaded by cloning.

5. Go to the folder using cd command.

6. Run the following command from terminal.
     
       cargo run
       
7. The output will be published successfully and you need to enter the column for the player.
       

8. The output will show winner whent it matches the four of one's player input.

Ref : https://en.wikipedia.org/wiki/Connect_Four
     
      https://github.com/michael-zucchetta/connect-four-rust-web-assembly
