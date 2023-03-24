import { createServer} from 'https'

import { HttpError, NotFoundError } from './errors.js'

import decodePosition from './api/decodePosition.js'
import decodeLoan from './api/decodeLoan.js'
import decodeProposalReceipts from './api/decodeProposalReceipts.js'
import decodeVoterCard from './api/decodeVoterCard.js'


import * as fs from 'fs'

const certPath = '/etc/letsencrypt/live/beaker.fi';

const options = {
  key: fs.readFileSync(`${certPath}/privkey.pem`),
  cert: fs.readFileSync(`${certPath}/fullchain.pem`)
};

export const server = createServer(options, async (req, res) => {
  try {
    res.setHeader('Content-Type', 'application/json')
    res.setHeader('Access-Control-Allow-Origin', '*')

    // Handle CORS preflight request
    if (req.method === 'OPTIONS') {
      res.setHeader('Access-Control-Allow-Headers', 'Content-Type, Accept, Origin, Authorization')
      res.setHeader('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE, PATCH, OPTIONS')
      return res.end()
    }

    const url = new URL(req.url!, 'https://' + req.headers.host!)
    const endpoint = `${req.method}:${url.pathname}`

    let result;
    switch (endpoint) {

      // Home
        case 'GET:/loan':
          result = await decodeLoan(url);
          break;

        case 'GET:/position':
          result = await decodePosition(url);
          break;   

        case 'GET:/proposal_receipt':
          result = await decodeProposalReceipts(url);
          break;   

        case 'GET:/voter_cards':
          result = await decodeVoterCard(url);
          break;      
    

        default:
            throw new NotFoundError('Endpoint not found');
    }
    if (result != undefined) {
      res.writeHead(200, { 'Content-Type': 'application/json' });
      res.write(JSON.stringify(result))
    } else {
      throw new Error("Empty Result")
    }
  } catch (e: any) {
    let status = 500
    let message = 'Internal Server Error'
    if (e instanceof HttpError) {
      status = e.status
      message = e.message
    } else {
      console.log('Undetermined Error:', e)
    }
    res.writeHead(status)
    res.write(JSON.stringify({ error: message }))
  } finally {
    res.end()
  }
})

server.listen(9999)

