// Import
const express = require('express');
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { Keyring } = require('@polkadot/keyring');
const { readFileSync } = require('fs');
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
        const keyring = new Keyring({ type: 'sr25519' });
        const alice = keyring.addFromUri('//Alice');
        const types = JSON.parse(readFileSync('./types.json', 'utf8'));
        const api = await ApiPromise.create({ provider: wsProvider,
            types
        });
        
        try{
            let namePromise = digestMessage("Aman Verma");
            namePromise.then((cust)=>{
                const addCust = api.tx.carpooling.addNewCustomer(15,{id: 15, name: "0x"+cust, location: [20,40]});
                addCust.signAndSend(alice);
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

app.get('/updatecust', (req, res) =>{

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
    // main functions calls the updatecustlocation dispatch function to update customer location.
    async function main(){


        // Construct
        const wsProvider = new WsProvider('ws://127.0.0.1:9944');
        const keyring = new Keyring({ type: 'sr25519' });
        const alice = keyring.addFromUri('//Alice');
        const types = JSON.parse(readFileSync('./types.json', 'utf8'));
        const api = await ApiPromise.create({ provider: wsProvider,
            types
        });

        try{
                const updateCustLocation = api.tx.carpooling.updateCustLocation(15,[40,20]);
                updateCustLocation.signAndSend(alice);
                console.log(`The customer location was updated successfully`);
            ;

        }
        catch(error){
            console.log(error);
        }
    }
    main().then(() => console.log('completed'));
    res.send("Done");
});
app.listen(6069);