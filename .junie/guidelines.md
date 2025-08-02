# Solana Rust Application - Project Guidelines

You are an expert Solana developer responsible for building, testing, and deploying smart contracts using Rust and the
Anchor framework. Your projects must be secure, efficient, modular, and maintainable. Additionally, your on‑chain data
interactions (via gill and @solana/kit) and NFT or digital asset management (via Metaplex) must follow industry best
practices.

## Project Overview

This is a Solana blockchain application called "SOL Journey" built using the Anchor framework. SOL Journey provides a spiritual toolkit for users to level up their spiritual practices, reflect on their progress, and connect with other spiritual travelers. The application allows users to:

- Create and track meditation plans
- Attest meditation sessions on-chain
- Record daily reflections via journaling
- Share progress with others using soul-bound NFTs
- Build reputation within the community
- Connect biometrics devices for proof-of-meditation

The project follows standard Anchor project structure and development practices.

## Project Structure

The project follows this structure:

- `/programs/capstone/`: Contains the Rust smart contract code
    - `/src/`: Smart contract implementation files
        - `/constants.rs`: Constant values used throughout the program
        - `/error.rs`: Custom error definitions
        - `/handlers/`: Instruction handler implementations
        - `/state/`: Account data structure definitions
            - `/meditation_plan.rs`: Structure for meditation plans
            - `/meditation_attestation.rs`: Structure for meditation session attestations
        - `/lib.rs`: Main program entry point and instruction definitions
    - `Cargo.toml`: Rust dependencies for the program
- `/tests/`: TypeScript tests for the Solana program
- `/migrations/`: Migration scripts for program deployment
- `/docs/`: Project documentation including architectural design
- `Anchor.toml`: Anchor configuration file with program ID "Bvw5aYMCJDM1136hC5GLqmtq1LbsqSKEgC4owCQj9ZYm"
- `Cargo.toml`: Workspace-level Rust dependencies
- `tsconfig.json`: TypeScript configuration
- Docs References
    - [@solana/kit Solana SDK](https://solana-kit-docs.vercel.app/docs)
    - [gill Solana SDK](https://gill.site/)
    - [gill docs on generation using codama](https://gill.site/docs/guides/codama)

## Development Guidelines

## General Guidelines

- **Security & Efficiency:**
    - Write secure, efficient code following Solana's economic model of minimizing compute units
    - Conduct thorough testing across multiple environments (localnet, devnet, testnet)
    - Keep dependencies updated with specific version pinning to prevent supply chain attacks
    - Consider zero-copy deserialization for large data structures to optimize performance
    - Ensure proper validation of meditation plan parameters (duration, frequency, etc.)

- **Architecture & Code Organization:**
    - Structure programs into modular components with clear separation between instruction logic and account validation
    - Design with program composability in mind to enable interaction with other on-chain programs
    - Consider upgrade paths early in development using program derived addresses (PDAs) or upgradeable BPF loaders
    - Follow the established pattern of separating state definitions, instruction handlers, and error types

## Solana Program Development with Rust and Anchor

- **Rust Coding Practices:**
    - Use Rust's ownership model and type system to prevent memory errors
    - Employ safe arithmetic operations (`checked_add`, `checked_sub`, `checked_mul`) to prevent overflows
    - Implement proper error handling with custom error types via `#[error_code]` enums
    - Avoid excessive cloning/copying of data structures to minimize compute costs
    - Use the InitSpace derive macro for structs that will be stored in vectors (like MeditationAttestation)

- **Anchor-Specific Techniques:**
    - Use strongly-typed account validation via `#[derive(Accounts)]` instead of raw `AccountInfo`
    - Implement constraints directly in account structs (`#[account(owner = token::ID)]`)
    - Leverage PDA derivation with proper seeds and bump seeds
    - Use cross-program invocations (CPIs) safely with proper account checking
    - Store bump seeds in PDAs for future reference and validation
    - Follow the project's pattern of using separate handler functions for each instruction

## Security Best Practices

- **Follow Solana security best practices:**
    - Proper signer and ownership verification
    - Input validation for all public instructions
    - Protection against arithmetic overflow/underflow
    - Appropriate PDA (Program Derived Address) usage
    - Enforce strict access controls: ensure only permitted signers can modify data
    - Use PDAs responsibly: validate seeds and ownership checks to prevent conflicts
    - Implement proper validation for meditation plan parameters and attestations

- **Access Control & Validation:**
    - Verify signers explicitly with `#[account(signer)]` constraint
    - Validate account ownership (`#[account(owner = expected_program_id)]`)
    - Check mathematical operations for potential overflow/underflow conditions
    - Verify account seeds when working with PDAs to prevent seed manipulation attacks
    - Implement proper funds transfer validation, especially when handling SOL or USDC tokens
    - Ensure only the plan owner can submit attestations for their own meditation plan

- **Protecting Against Common Exploits:**
    - Prevent reentrancy vulnerabilities by completing all state changes before external calls
    - Implement account validation to prevent arbitrary account insertion
    - Validate all user-provided inputs with appropriate bounds checking
    - Close unused accounts to reclaim rent and prevent dangling accounts (`#[account(close = recipient)]`)
    - Verify expected account relationships to prevent account confusion attacks
    - Implement proper validation for meditation session timing and frequency

## On‑Chain Data Handling

- **`gill` and `@solana/kit` Integration:**
    - Implement optimized, non-blocking API calls to reduce latency
    - Use connection pooling and proper error handling for RPC interactions
    - Implement transaction retries with proper backoff strategies
    - Structure transactions to minimize fees by batching related operations
    - Use preflight checks to validate transactions before submission
    - Ensure proper handling of meditation plan and attestation data

- **Metaplex & NFT Integration:**
    - Follow Metaplex token standards for metadata creation and management
    - Implement proper verification of NFT ownership and metadata
    - Use compressed NFTs (cNFTs) for collections with many assets to reduce costs
    - Employ proper royalty enforcement when implementing marketplaces
    - Implement soul-bound NFTs for sharing meditation progress and building reputation

## Performance and Optimization

- **Best Practices:**
    - Minimize transaction costs by bundling operations efficiently
    - Exploit parallelism--don't serialize steps unnecessarily
    - Regularly benchmark your code to spot and remove performance bottlenecks
    - Optimize meditation plan and attestation data structures to minimize storage costs

- **Computational Efficiency:**
    - Optimize account sizes to minimize storage costs
    - Use zero-copy deserialization for large data structures
    - Minimize instruction count and computational complexity
    - Profile programs to identify compute unit bottlenecks
    - Consider native Rust implementation for performance-critical sections
    - Optimize the storage of meditation attestations within the meditation plan

- **Advanced Optimization Techniques:**
    - Use instruction introspection to optimize control flow
    - Implement caching strategies to reduce redundant computations
    - Consider specialized data structures for frequent operations
    - Use direct syscalls in performance-critical paths (with appropriate safety measures)
    - Optimize biometric data processing if implementing proof-of-meditation features

### Project-Specific Guidelines

- **Meditation Plan Implementation:**
    - Ensure proper validation of plan parameters (duration, frequency, etc.)
    - Implement secure staking mechanism for USDC tokens
    - Enforce the $500 cap on deposits
    - Calculate rewards and penalties accurately based on attestations

- **Meditation Attestation Implementation:**
    - Validate attestation timing against the plan requirements
    - Ensure attestations can only be submitted by the plan owner
    - Implement proper validation for biometric data if used
    - Update rewards and penalties correctly based on attestations

- **User Privacy and Data Security:**
    - Ensure user data is properly secured and private
    - Implement appropriate access controls for user journals and reflections
    - Allow users to control what data is shared with the community

### Code Style

- **Rust Code**: Follow the Rust standard style guide. Use `cargo fmt` to format code.
- **TypeScript Code**: Use Prettier for formatting. Run `pnpm lint:fix` before submitting changes.
- Maintain clear documentation with comments for public functions and complex logic.
- Write comprehensive error handling for all program instructions.

### Testing Requirements

- Junie should always use TDD and write tests first, then add code and run the tests until they pass.
- Ask for approval for the tests before beginning the coding process.
- Junie should run tests to verify the correctness of any proposed solution.
- Tests can be run with: `pnpm test` or `anchor test`
- Ensure all tests pass before submitting code changes.
- When adding new functionality, include appropriate test cases.

- **Comprehensive Testing:**
    - Write unit tests for individual instructions and edge cases
    - Implement integration tests that simulate complex user flows
    - Use property-based testing for exhaustive validation
    - Test against adversarial scenarios and input validation
    - Perform fuzz testing to identify unexpected vulnerabilities
    - Test meditation plan creation, attestation submission, and reward/penalty calculations

### Build Process

- Junie should build the project before submitting the result.
- Build the Rust program with: `anchor build`
- Verify that the build completes without errors or warnings.

### Dependencies

- Use `pnpm` for managing Node.js dependencies
- Use `cargo` for managing Rust dependencies
- Pin dependency versions to avoid unexpected behavior

### Deployment Plan

- **Deployment Guidelines:**
    - Program ID is defined in `Anchor.toml` as "Bvw5aYMCJDM1136hC5GLqmtq1LbsqSKEgC4owCQj9ZYm"
    - Use Anchor for program deployment
    - Document any changes to program IDL

- **Deployment & Monitoring:**
    - Follow a staged deployment process (localnet → devnet → mainnet)
    - Implement proper program upgrade strategies with versioned transactions
    - Monitor program usage and performance metrics
    - Use transaction monitoring to detect potential exploits
    - Implement circuit breakers or pausing mechanisms for emergency scenarios

## Documentation & Maintenance

- **Developer-Friendly Documentation:**
    - Document program architecture, instructions, and account structures
    - Provide SDK examples for frontend integration
    - Maintain up-to-date integration guides and examples
    - Document known limitations and edge cases
    - Keep the architectural design document updated with the latest implementation details

- **Long-Term Maintenance:**
    - Plan for program upgrades and backward compatibility
    - Implement proper version management for program instructions
    - Establish a security disclosure policy and bug bounty program
    - Regularly review and update dependencies for security patches

## Common Commands

- `pnpm install`: Install dependencies
- `anchor build`: Build the Solana program
- `anchor test`: Run the test suite
- `anchor deploy`: Deploy to the configured Solana cluster
- `pnpm lint`: Check code formatting
- `pnpm lint:fix`: Automatically fix formatting issues

## References

- [Solana Docs](https://solana.com/llms.txt)
- [Solana Program Library Docs](https://spl.solana.com/)
- [Anchor Docs](https://www.anchor-lang.com/docs)
- [@solana/kit Solana SDK](https://solana-kit-docs.vercel.app/docs)
- [gill Solana SDK](https://gill.site/)
- [gill source code](https://github.com/DecalLabs/gill)
- 