import 'regenerator-runtime/runtime'
import React from 'react'
import { login, logout } from './utils'
import './global.css'
import { Contract } from 'near-api-js'

import getConfig from './config'
const { networkId } = getConfig(process.env.NODE_ENV || 'development')

const REF_EXCHANGE_CONTRACT_ID = "exchange.ref-dev.testnet";
const SEED_ID = ":107";
const RECEIVER_ID = 'xpanse-core.testnet';
const GAS_300 = "300000000000000";
const YOCTO_NEAR_1 = "1";

export default function App() {
  // use React Hooks to store greeting in component state
  const [greeting, set_greeting] = React.useState()

  // when the user has not yet interacted with the form, disable the button
  // const [buttonDisabled, setButtonDisabled] = React.useState(true)

  // after submitting the form, we want to show Notification
  const [showNotification, setShowNotification] = React.useState(false)

  // The useEffect hook can be used to fire side-effects during render
  // Learn more: https://reactjs.org/docs/hooks-intro.html
  React.useEffect(
    () => {
      // in this case, we only care to query the contract when signed in
      if (window.walletConnection.isSignedIn()) {
        set_greeting("0.000001")
        // window.contract is set by initContract in index.js
        // window.contract.get_greeting({ account_id: window.accountId })
        //   .then(greetingFromContract => {
        //    set_greeting(greetingFromContract)
        //   })
      }
    },

    // The second argument to useEffect tells React when to re-run the effect
    // Use an empty array to specify "only run on first render"
    // This works because signing into NEAR Wallet reloads the page
    []
  )

  // if not signed in, return early with sign-in prompt
  if (!window.walletConnection.isSignedIn()) {
    return (
      <main>
        <h1>Welcome to XPANSE!</h1>
        <p>
          To make use of the NEAR blockchain, you need to sign in. The button
          below will sign you in using NEAR Wallet.
        </p>
        <p style={{ textAlign: 'center', marginTop: '2.5em' }}>
          <button onClick={login}>Sign in</button>
        </p>
      </main>
    )
  }

  return (
    // use React Fragment, <>, to avoid wrapping elements in unnecessary divs
    <>
      <button className="link" style={{ float: 'right' }} onClick={logout}>
        Sign out
      </button>
      <main>
        <h1>
          <label
            htmlFor="greeting"
            style={{
              color: 'var(--secondary)',
              borderBottom: '2px solid var(--secondary)'
            }}
          >
          </label>
          {' '/* React trims whitespace around tags; insert literal space character when needed */}
          Hi {window.accountId}
        </h1>

        {/* ---------------------------------------------------------------------- */}
        {/* Deposit Seeds Form */}
        {/* ---------------------------------------------------------------------- */}
        <form onSubmit={async event => {
          event.preventDefault()

          // get elements from the form using their id attribute
          const { fieldset, greeting } = event.target.elements

          // hold onto new user-entered value from React's SynthenticEvent for use after `await` call
          var deposit_amount = greeting.value
          deposit_amount = (parseFloat(deposit_amount) * (10 ** 24)).toString()
          console.log(deposit_amount)

          // disable the form while the value gets updated on-chain
          // fieldset.disabled = true

          const exchange_contract = new Contract(
            window.walletConnection.account(), // the account object that is connecting
            REF_EXCHANGE_CONTRACT_ID,
            {
              viewMethods: [], // view methods do not change state but usually return a value
              changeMethods: ["mft_transfer_call"], // change methods modify state
            }
          );
          try {
            // make an update call to the smart contract
            await exchange_contract.mft_transfer_call(
              {
                token_id: SEED_ID,
                receiver_id: RECEIVER_ID,
                amount: deposit_amount, // Minimum required => '1000000000000000000'
                msg: ''
              },
              GAS_300, // attached GAS 
              YOCTO_NEAR_1 // attached deposit in yoctoNEAR
            )

          } catch (e) {
            alert(
              'Something went wrong! ' +
              'Maybe you need to sign out and back in? ' +
              'Check your browser console for more info.'
            )
            throw e
          } finally {
            // re-enable the form, whether the call succeeded or failed
            // fieldset.disabled = false
          }

          // update local `greeting` variable to match persisted value
          // set_greeting(newGreeting)

          // show Notification
          setShowNotification(true)

          // remove Notification again after css animation completes
          // this allows it to be shown again next time the form is submitted
          setTimeout(() => {
            setShowNotification(false)
          }, 11000)
        }}>
          <fieldset id="fieldset">
            <label
              htmlFor="greeting"
              style={{
                display: 'block',
                color: 'var(--gray)',
                marginBottom: '0.5em'
              }}
            >
              Deposit Seeds ( Min 0.000001 )
            </label>
            <div style={{ display: 'flex' }}>
              <input
                autoComplete="off"
                defaultValue={greeting}
                id="greeting"
                // onChange={e => setButtonDisabled(e.target.value === greeting)}
                style={{ flex: 1 }}
              />
              <button
                // disabled={buttonDisabled}
                style={{ borderRadius: '0 5px 5px 0' }}
              >
                Deposit Seeds
              </button>
            </div>
          </fieldset>
        </form>

        {/* ---------------------------------------------------------------------- */}
        {/* Withdraw Seeds Form */}
        {/* ---------------------------------------------------------------------- */}
        <form onSubmit={async event => {
          event.preventDefault()

          // get elements from the form using their id attribute
          const { fieldset, greeting } = event.target.elements

          // hold onto new user-entered value from React's SynthenticEvent for use after `await` call
          var withdraw_amount = greeting.value
          withdraw_amount = (parseFloat(withdraw_amount) * (10 ** 24)).toString()
          withdraw_amount = withdraw_amount.toString()
          console.log(withdraw_amount)

          // disable the form while the value gets updated on-chain
          // fieldset.disabled = true

          try {
            // make an update call to the smart contract
            await window.contract.withdraw(
              {
                amount: withdraw_amount, // Minimum required => '1000000000000000000'
              },
              GAS_300, // attached GAS
            )

          } catch (e) {
            alert(
              'Something went wrong! ' +
              'Maybe you need to sign out and back in? ' +
              'Check your browser console for more info.'
            )
            throw e
          } finally {
            // re-enable the form, whether the call succeeded or failed
            // fieldset.disabled = false
          }

          // update local `greeting` variable to match persisted value
          // set_greeting(newGreeting)

          // show Notification
          setShowNotification(true)

          // remove Notification again after css animation completes
          // this allows it to be shown again next time the form is submitted
          setTimeout(() => {
            setShowNotification(false)
          }, 11000)
        }}>
          <fieldset id="fieldset">
            <label
              htmlFor="greeting"
              style={{
                display: 'block',
                color: 'var(--gray)',
                marginBottom: '0.5em'
              }}
            >
              Withdraw Seeds
            </label>
            <div style={{ display: 'flex' }}>
              <input
                autoComplete="off"
                defaultValue={greeting}
                id="greeting"
                // onChange={e => setButtonDisabled(e.target.value === greeting)}
                style={{ flex: 1 }}
              />
              <button
                // disabled={buttonDisabled}
                style={{ borderRadius: '0 5px 5px 0' }}
              >
                Withdraw Seeds
              </button>
            </div>
          </fieldset>
        </form>

        {/* ---------------------------------------------------------------------- */}
        {/* Claim Seeds Form */}
        {/* ---------------------------------------------------------------------- */}
        <form onSubmit={async event => {
          event.preventDefault()

          try {
            // make an update call to the smart contract
            await window.contract.claim(
              {},
              GAS_300, // attached GAS
            )

          } catch (e) {
            alert(
              'Something went wrong! ' +
              'Maybe you need to sign out and back in? ' +
              'Check your browser console for more info.'
            )
            throw e
          } finally { }

          // show Notification
          setShowNotification(true)

          // remove Notification again after css animation completes
          // this allows it to be shown again next time the form is submitted
          setTimeout(() => {
            setShowNotification(false)
          }, 11000)
        }}>
          <fieldset id="fieldset">
            <label
              htmlFor="greeting"
              style={{
                display: 'block',
                color: 'var(--gray)',
                marginBottom: '0.5em'
              }}
            >
              Claim Amount
            </label>
            <div style={{ display: 'flex' }}>
              { }
              <button
                style={{ borderRadius: '0 5px 5px 0' }}
              >
                Claim Amount
              </button>
            </div>
          </fieldset>
        </form>
        <p>
          The Xpanse can manage / execute an end-to-end auto-compounding strategy on Ref Finance.
        </p>
        <ol>
          <li>
            Deposit your Seeds(LP Tokens) into Xpanse Contract.
            The Contract will execute auto-compounding strategy including staking of seeds, claiming rewards, swapping tokens and add Liquidity.
          </li>
          <li>
            Withdraw Your Seeds Whenever required.
          </li>
          <li>
            Claim the withdrawn amount and transfer the seeds back to your account.
          </li>
        </ol>
        <hr />
      </main>
      {showNotification && <Notification />}
    </>
  )
}

// this component gets rendered by App after the form is submitted
function Notification() {
  const urlPrefix = `https://explorer.${networkId}.near.org/accounts`
  return (
    <aside>
      <a target="_blank" rel="noreferrer" href={`${urlPrefix}/${window.accountId}`}>
        {window.accountId}
      </a>
      {' '/* React trims whitespace around tags; insert literal space character when needed */}
      called method: 'set_greeting' in contract:
      {' '}
      <a target="_blank" rel="noreferrer" href={`${urlPrefix}/${window.contract.contractId}`}>
        {window.contract.contractId}
      </a>
      <footer>
        <div>✔ Succeeded</div>
        <div>Just now</div>
      </footer>
    </aside>
  )
}
