pragma solidity ^0.8.24;

/// @title Simple voting mechanism - supports delegation.
contract Ballot {
    // Field: a voter.
    struct Voter {
        // Weight accumulated by delegation.
        uint weight;
        // Flag indicating whether entity has voted.
        bool voted;
        // Entity to which vote is to be delegated.
        address delegate;
        // Voting proposal indentifier.
        uint vote;
    }

    // Field: a proposal that may result in a vote.
    struct Proposal {
        bytes32 name;
        uint voteCount;
    }

    // Field: Entity responsible for voting coordination.
    address public chairPersion;

    // Field: Mapping of registered addresses to voters.
    mapping(address => Voter) public voters;

    // Field: Array of submitted proposals.
    Proposal[] public proposals;

    // Ctor: accept a set of proposal names/identifiers.
    constructor(bytes32[] memory proposalNames) {
        chairPersion = msg.sender;
        voters[chairPersion].weight = 1;

        for (uint i = 0; i < proposalNames.length; i++) {
            proposals.push(Proposal({name: proposalNames[i], voteCount: 0}));
        }
    }

    // Mtor: Grants voting rights to an entity.
    function giveRightToVote(address voter) external {
        // Guard.
        require(
            msg.sender == chairPersion,
            "Only chair person can grant voting rights"
        );
        require(
            voters[voter].voted == false,
            "Voter is attempting to vote twice"
        );
        require(voters[voter].weight == 0, "Voter weight is not zero");

        // Set voter's vote permissions - i.e. weight.
        voters[voter].weight = 1;
    }

    // Mtor: Delegates voting rights to a 3rd party entity.
    function delegate(address to) external {
        // Guard.
        require(to != msg.sender, "Self-delegation is not allowed");

        Voter storage sender = voters[msg.sender];
        require(sender.weight != 0, "Insufficient voting permissions");
        require(sender.voted == false, "Duplicate vote");

        while (voters[to].delegate != address(0)) {
            to = voters[to].delegate;
            require(to != msg.sender, "Found loop in delegation chain");
        }

        Voter storage delegatee = voters[to];
        require(delegatee.weight >= 1, "Delegatee cannot vote");

        // Mutate state.
        sender.delegate = to;
        sender.voted = true;
        if (delegatee.voted == true) {
            proposals[delegatee.vote].voteCount += sender.weight;
        } else {
            delegatee.weight += sender.weight;
        }
    }

    // Mtor: Exercises vote.
    function vote(uint proposal) external {
        // Guard.
        Voter storage sender = voters[msg.sender];
        require(sender.weight != 0, "Insufficient voting permissions");
        require(sender.voted == false, "Duplicate voting");

        // Mutate state.
        sender.voted = true;
        sender.vote = proposal;
        proposals[proposal].voteCount += sender.weight;
    }

    // View: Returns wining proposal identifer.
    function winningProposal() public view returns (uint winningProposal_) {
        uint winningVoteCount = 0;
        for (uint p = 0; p < proposals.length; p++) {
            if (proposals[p].voteCount > winningVoteCount) {
                winningVoteCount = proposals[p].voteCount;
                winningProposal_ = p;
            }
        }
    }
}
