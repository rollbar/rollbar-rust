// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import type { NextApiRequest, NextApiResponse } from 'next'

//const Rollbar = require('rollbar-node')

//const rollbar = new Rollbar({
  //accessToken: process.env.POST_TOKEN,
//})

type Data = {
  randomWord: string
}

export default function handler(
  req: NextApiRequest,
  res: NextApiResponse<Data>
) {
  res
    .status(200)
    .json({ randomWord: 'gabagoo' })
}
