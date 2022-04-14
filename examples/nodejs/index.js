const Rollbar = require('rollbar-node')

const rollbar = new Rollbar({
  accessToken: process.env.POST_TOKEN,
})

rollbar.log('warning', 'oopsie', {
  some: 'stuff'
})

rollbar.shutdown();
