use std::cmp::Ordering;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use rand::Rng;

fn main() {
    //create a TcpListener which bind to 7878 port
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //the main for-loop to receive incoming streams
    //log the listener if successful
    for stream in listener.incoming() {
        //for everything incoming stream, use match to deal with
        match stream {
            //if stream  incoming successful
            Ok(stream) => {
                //then we handle it
                handle_connection(stream);
            }
            //otherwise, when there is error
            Err(e) => {
                //we just print it out
                println!("something wrong here {}", e);
            }
        };
    }
}
// the handle function to deal with each incoming stream
fn handle_connection(mut stream: TcpStream) {
    //standard u8 buffer with size of 1024, since some of the signal like ctrl+c is actually much
    //biger incoming, 1024 is a good start.
    let mut buffer = [0; 1024];
    //generate a random number between 1 and 100(include),
    let secret_number = rand::thread_rng().gen_range(1..101);
    //log in server side, so we can know which number had generated, this only for sever side
    println!(
        "well, we got a player, and we set secret number {} here",
        secret_number
    );
    // the welcom information
    let welcom = r#"
    ****************************************
   
    Welcome to TCP Guess Game,
    Please input a number below as your guess,
    number should between 1 and 100,
    
    or,

    you can just press Enter to fast quit~
    
    ****************************************

    Please input Your Guess: "#;
    //sent the welcome information to client side for print
    stream.write_all(welcom.as_bytes()).unwrap();
    //the main dealing loop, if player guess the exact number, or press Enter without any input, or
    //some other signal like press "Ctr + C", the loop will break itself out, and close the
    //connection, client side will have to re-connect again to play
    loop {
        //read the client's input stream into buffer
        stream.read(&mut buffer).unwrap();
        //generated a Cow<str> from buffer
        let input = String::from_utf8_lossy(&buffer[..]);
        //no matter what input, just print it out in server side
        println!("Player inputs {}", input);
        // try to get the 1st line from the input, this won't failure, so unwrap is saft to use.
        let input = input.lines().next().unwrap();
        //the 1st line is empty means the player press Enter without any normal input(digital or
        //chars, or might be signal inputs like "Ctr-C", which should mean to close the connection,
        //however, the Enter press without inputs might be mistake by player, and a bit confusse,
        //so we tell the player in the Welcome information for tips.
        if input.is_empty() {
            //log those inputs in server side
            println!("\nGot nothing, close the connection~\n");
            // flush the stream for sake of next incoming
            stream.flush().unwrap();
            //use break to quit the loop and therefore close the connection
            break;
        }
        //try parse input into a u32 guess number
        let guess: u32 = match input.trim().parse() {
            //player's input is correct, parse ok
            Ok(num) => num,
            // input is not a number, we tell the client and let them try again
            Err(_) => {
                // tell the player that input should be number
                stream
                    .write_all("\nNO! input must be a number! \nPlease input again: ".as_bytes())
                    .unwrap();
                //flush the write for next incoming's sake
                stream.flush().unwrap();
                //use continue to let player try again
                continue;
            }
        };
        //print the input guess
        println!("player guessed : {}", guess);
        //now, player input number and we are going to compare to the secret random number we have
        //generated
        match guess.cmp(&secret_number) {
            //use Ordering enum to deal with, if Ordering::Less, means player's guess is small than
            //our secret
            Ordering::Less => {
                // tell player the guess is too small
                stream
                    .write_all("\nToo small! \ntry again: ".as_bytes())
                    .unwrap();
                //let player try again
                continue;
            }
            //Ordering::Greater means guess is bigger than secret
            Ordering::Greater => {
                // the player the guess is too big
                stream
                    .write_all("\nToo big! \ntry again: ".as_bytes())
                    .unwrap();
                // let player try again
                continue;
            }
            // if Ordering::Equal, means player successful guess the exact number,
            Ordering::Equal => {
                //log in server
                println!(
                    "player wins this time, close connection and waiting for next player....\n"
                );
                // Tell player the good news,
                stream
                    .write_all(
                        "\nYou Win The Guess! Well Done, \nnow I have to say Good-bye!! \n\n"
                            .as_bytes(),
                    )
                    .unwrap();
                //this time, we close the connection, for player has win the game, we have to say
                //goodbye...
                break;
            }
        }
    }
}
