/** @jsxImportSource theme-ui */
import { useNavigate } from "react-router-dom";

import { useContext, useState } from "react";

import Dashboard from "components/Dashboard";

import { randomIntFromInterval } from "utils/general/generalMaths";

import Star from "components/Star";
import Snackbar from "components/Snackbar";
import ConnectWallet2 from "components/ConnectWalletLarge";

import { UserContext } from "contexts/UserContext";
import { ResponsiveContext } from "contexts/ResponsiveContext";

import styleFunction from "./style";
import { TokensContext } from "contexts/TokensContext";
import { proposal } from "types";

import { voteForProposal, voteAgainstProposal } from "utils/dao/daoContractCalls";

function Dao() {
    const [stars, setStars] = useState(Array.from({ length: 10 }, (_, i) => [randomIntFromInterval(0, 1), randomIntFromInterval(10, 90), randomIntFromInterval(10, 90), randomIntFromInterval(0, 1)]));

    const { user, voterCard } = useContext(UserContext);
    const { device } = useContext(ResponsiveContext);

    const { dao } = useContext(TokensContext);

    const possibleChoices = ["Change Vote Period", "Change Minimum Vote Threshold", "Allow Claim", "Add New Collateral Token", "Change Lender Parameters", "Change Lender Oracle", "Add Tokens To Issuers Reserves"]

    const [addProposal, setAddProposal] = useState<boolean>(false);

    const [expand, setExpand] = useState<boolean>(false);

    const [currentIndex, setCurrentIndex] = useState<number>(0);

    function toggleExpand() {
        setExpand(!expand);
    }

    function toggleProposal() {
        setAddProposal(!addProposal);
    }

    const style = styleFunction(device, expand);

    const currentUnix = Date.now() / 1000;

    return (
        <Dashboard page='dao'>
            <Snackbar />

            {stars.map((x, index) => {
                return (
                    <Star key={"star" + index} left={x[1].toString()} top={x[2].toString()} height={x[0] ? "15" : "20"} color={x[3] ? "text" : "text2"} />
                )
            })}


            <div sx={style.main}>

                <div sx={style.votesContainer}>
                    <button sx={style.add} onClick={toggleProposal}>
                        {addProposal ? "See Proposals" : "Add A Proposal"}
                    </button>

                    {addProposal ?

                        <div sx={style.addProposalZone}>
                            <label htmlFor="title">Title</label>
                            <input type="text" id="title" />
                            <label htmlFor="subtitle">Description</label>
                            <textarea id="subtitle" />

                            <div sx={style.property} onClick={toggleExpand}>
                                {possibleChoices[currentIndex]}<div sx={expand ? style.expand : style.expand2} />
                            </div>
                            {expand ?
                                <div sx={style.possibleChoices}>
                                    {possibleChoices.map((x: string, index: number) => {
                                        return (
                                            <p key={"choice" + index} onClick={() => { setCurrentIndex(index); setExpand(false); }}>{x}</p>
                                        )
                                    })}
                                </div>
                                :
                                null
                            }

                            {currentIndex == 0 ?
                                <div sx={{ width: '100%' }}>
                                    <label htmlFor="seconds">How many seconds ?</label>
                                    <input type="text" id="seconds" />
                                </div>
                                : currentIndex == 1 ?
                                    <div sx={{ width: '100%' }}>
                                        <label htmlFor="minimum">Percentage Needed ?</label>
                                        <input type="number" id="minimum" min="0" max="100" step="0.01" />
                                    </div>
                                    : currentIndex == 2 ?
                                        <div sx={{ width: '100%' }}>
                                            <label htmlFor="tokenAddress">Token Address</label>
                                            <input type="text" id="tokenAddress" />
                                            <label htmlFor="claim">How Many Tokens to Claim ?</label>
                                            <input type="number" id="claim" />
                                        </div>
                                        : currentIndex == 3 ?
                                            <div sx={{ width: '100%' }}>
                                                <label htmlFor="tokenAddress">Token Address</label>
                                                <input type="text" id="tokenAddress" />
                                                <label htmlFor="LTV">Loan To Value (between 0 and 1)</label>
                                                <input type="number" id="LTV" min="0" max="1" step=".01" />
                                                <label htmlFor="interestRate">Interest Rate (between 0 and 1)</label>
                                                <input type="number" id="interestRate" min="0" max="1" step=".01" />
                                                <label htmlFor="LT">Liquidiation Threshold (between 1 and 1/LTV)</label>
                                                <input type="number" id="LP" min="0" step=".01" />
                                                <label htmlFor="LT">Linquidation Penalty (between 0 and 1)</label>
                                                <input type="number" id="LP" min="0" max="1" step=".01" />
                                                <label htmlFor="IP">Initial Price</label>
                                                <input type="number" id="IP" />
                                                <label htmlFor="mP">Minimum Price</label>
                                                <input type="number" id="mP" />
                                                <label htmlFor="MP">Maximum Price</label>
                                                <input type="number" id="MP" />
                                                <label htmlFor="oracleAddress">Pool Oracle Address</label>
                                                <input type="text" id="oracleAddress" />
                                            </div>
                                            : currentIndex == 4 ?
                                                <div sx={{ width: '100%' }}>
                                                    <label htmlFor="tokenAddress">Token Address</label>
                                                    <input type="text" id="tokenAddress" />
                                                    <label htmlFor="LTV">New Loan To Value (between 0 and 1)</label>
                                                    <input type="number" id="LTV" min="0" max="1" step=".01" />
                                                    <label htmlFor="interestRate">New Interest Rate (between 0 and 1)</label>
                                                    <input type="number" id="interestRate" min="0" max="1" step=".01" />
                                                    <label htmlFor="LT">New Liquidiation Threshold (between 1 and 1/LTV)</label>
                                                    <input type="number" id="LP" min="0" step=".01" />
                                                    <label htmlFor="LT">New Linquidation Penalty (between 0 and 1)</label>
                                                    <input type="number" id="LP" min="0" max="1" step=".01" />
                                                </div>
                                                : currentIndex == 5 ?
                                                    <div sx={{ width: '100%' }}>
                                                        <label htmlFor="tokenAddress">Token Address</label>
                                                        <input type="text" id="tokenAddress" />
                                                        <label htmlFor="oracleAddress">New Oracle Address</label>
                                                        <input type="text" id="oracleAddress" />
                                                    </div>
                                                    : currentIndex == 6 ?
                                                        <div sx={{ width: '100%' }}>
                                                            <label htmlFor="tokenAddress">Token Address</label>
                                                            <input type="text" id="tokenAddress" />
                                                            <label htmlFor="amount">How Many Tokens to Add ?</label>
                                                            <input type="number" id="amount" />
                                                        </div>
                                                        :
                                                        null
                            }

                            {user.address ? <button sx={style.send}>Submit</button> : <ConnectWallet2 />}
                        </div>

                        : null

                    }

                    {!addProposal ? dao.proposals.map((x: proposal, index: number) => {

                        return (

                            <div key={"dao" + index} sx={style.voteContainer}>
                                <div sx={style.vote}>
                                    <div sx={style.column}>
                                        <h3>No Title</h3>
                                        <h4>No Subtitle</h4>
                                        <p sx={style.date}>{x.vote_end < currentUnix ? "Vote" : "Vote ending in"} {x.vote_end < currentUnix ? x.votes_for > Math.max(x.votes_against, x.votes_threshold) ? <span sx={style.approved}>approved</span> : <span sx={style.declined}>declined</span> : <span>{x.vote_end - currentUnix} seconds</span>}</p>
                                        <div sx={style.caracteristics}>
                                            <ul>
                                                {x.proposed_change_data.map((y, index2) => {
                                                    return (
                                                        <li key={index + "proposal" + index2}>
                                                            {x.proposed_change_data[index2]}
                                                        </li>
                                                    )
                                                })}
                                            </ul>
                                        </div>
                                    </div>
                                    <div sx={style.score}>
                                        <p>{x.votes_for - x.votes_against}</p>
                                    </div>
                                </div>

                                {x.vote_end < currentUnix ? null :
                                    <div sx={style.voteButtons}>
                                        <button>+</button>
                                        <button>-</button>
                                    </div>
                                }
                            </div>
                        )
                    })
                        : null
                    }
                </div>
            </div>
        </Dashboard>
    )
}

export default Dao;