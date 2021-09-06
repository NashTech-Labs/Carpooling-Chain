// Import
const express = require('express');
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { Keyring } = require('@polkadot/keyring');
const { readFileSync } = require('fs');
var crypto = require('crypto');

const app = express();

app.get('/index', (req, res) =>{
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
            {
                const makeCabIdle = api.tx.carpooling.makeCabIdle(15);
                makeCabIdle.signAndSend(alice);
                console.log(`The cab was made Idle Successfully`);
            }

        }
        catch(error){
            console.log(error);
        }
    }
    main().then(() => console.log('completed'));
    res.send("Done");
});

app.listen(6069);
