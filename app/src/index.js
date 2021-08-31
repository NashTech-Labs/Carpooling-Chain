// Import
const express = require('express');
const { ApiPromise, WsProvider } = require('@polkadot/api');

var crypto = require('crypto');

const app = express();

app.get('/index', (req, res) =>{

    // digestMessage function converts a string to H256 hash string.
    //
    // # Arguments
    //
    // * `message` - A string parameter containing data to be converted.
    //
    // # Return
    //
    // A string containing hash value.

    async function digestMessage(message) {
        try{
            
            const hash = await crypto.createHash('sha256',message).digest('hex');
            return hash;
        }

        catch(error){
            console.log(error);
        }
        
      }
    // main functions calls the addNewCab dispatch function to add a new cab.
    async function main(){
    
    
        // Construct
        const wsProvider = new WsProvider('ws://127.0.0.1:9944');
    
        const api = await ApiPromise.create({ provider: wsProvider,
            types: {
                DriverOf: {
                    id: 'u32',
                    car_no: 'Hash',
                    location: ('u32', 'u32'),
                    price: 'u32',
                    destination: ('u32', 'u32')
                },
                CustomerOf: {
                    id: 'u32',
                    name: 'Hash',
                    location: ('u32', 'u32')
                },
              }
        });
        
        try{
            let namePromise = digestMessage("Ayushi Dwivedi");
            namePromise.then((car_no)=>{
                const addCab = api.tx.carpooling.addNewCab(15,{id: 15, car_no: car_no , location: (20,40), price: 30 , destination:(40,20)});
                addCab.signAndSend(alice);
                console.log(`The customer was successfully added`);
            });
            
        }
        catch(error){
            console.log(error);
        }  
    }   
    main().then(() => console.log('completed'));
    res.send("Done");
});

app.listen(6069);
