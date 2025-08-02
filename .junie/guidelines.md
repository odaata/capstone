# Solana Rust Application - Project Guidelines

You are an expert Solana developer responsible for building, testing, and deploying smart contracts using Rust and the
Anchor framework. Your projects must be secure, efficient, modular, and maintainable. Additionally, your on‑chain data
interactions (via gill and @solana/kit) and NFT or digital asset management (via Metaplex) must follow industry best
practices.

## Project Overview

This is a Solana blockchain application built using the Anchor framework.
The project follows standard Anchor project structure and development practices.

## Project Structure

The `anchor` programs are stored in the `/anchor` directory and follow the standard `anchor` project structure:

- `/programs/sol-journey/`: Contains the Rust smart contract code
    - `/src/`: Smart contract implementation files
    - `Cargo.toml`: Rust dependencies for the program
- `/tests/`: TypeScript tests for the Solana program
- `/migrations/`: Migration scripts for program deployment
- `Anchor.toml`: Anchor configuration file
- `Cargo.toml`: Workspace-level Rust dependencies
- `tsconfig.json`: TypeScript configuration
- `/src`: Contains a generated client using `gill` and `codama`.
- Docs References
    - [@solana/kit Solana SDK](https://solana-kit-docs.vercel.app/docs)
    - [gill Solana SDK](https://gill.site/)
    - [gill docs on generation using codama](https://gill.site/docs/guides/codama)

There is also a web dapp stored in the `/src` directory in the root.

- Next.js app using React
- Within the `/src` directory, the standard next.js structure is used.
- The web dapp also uses the generated `gill` client to interact with the programs.

## Development Guidelines

## General Guidelines

- **Security & Efficiency:**
    - Write secure, efficient code following Solana's economic model of minimizing compute units
    - Conduct thorough testing across multiple environments (localnet, devnet, testnet)
    - Keep dependencies updated with specific version pinning to prevent supply chain attacks
    - Consider zero-copy deserialization for large data structures to optimize performance

- **Architecture & Code Organization:**
    - Structure programs into modular components with clear separation between instruction logic and account validation
    - Design with program composability in mind to enable interaction with other on-chain programs
    - Consider upgrade paths early in development using program derived addresses (PDAs) or upgradeable BPF loaders

## Solana Program Development with Rust and Anchor

- **Rust Coding Practices:**
    - Use Rust's ownership model and type system to prevent memory errors
    - Employ safe arithmetic operations (`checked_add`, `checked_sub`, `checked_mul`) to prevent overflows
    - Implement proper error handling with custom error types via `#[error_code]` enums
    - Avoid excessive cloning/copying of data structures to minimize compute costs

- **Anchor-Specific Techniques:**
    - Use strongly-typed account validation via `#[derive(Accounts)]` instead of raw `AccountInfo`
    - Implement constraints directly in account structs (`#[account(owner = token::ID)]`)
    - Leverage PDA derivation with proper seeds and bump seeds
    - Use cross-program invocations (CPIs) safely with proper account checking
    - Store bump seeds in PDAs for future reference and validation

## Security Best Practices

- **Follow Solana security best practices:**
    - Proper signer and ownership verification
    - Input validation for all public instructions
    - Protection against arithmetic overflow/underflow
    - Appropriate PDA (Program Derived Address) usage
    - Enforce strict access controls: ensure only permitted signers can modify data.
    - Use PDAs responsibly: validate seeds and ownership checks to prevent conflicts.

- **Access Control & Validation:**
    - Verify signers explicitly with `#[account(signer)]` constraint
    - Validate account ownership (`#[account(owner = expected_program_id)]`)
    - Check mathematical operations for potential overflow/underflow conditions
    - Verify account seeds when working with PDAs to prevent seed manipulation attacks
    - Implement proper funds transfer validation, especially when handling SOL or SPL tokens

- **Protecting Against Common Exploits:**
    - Prevent reentrancy vulnerabilities by completing all state changes before external calls
    - Implement account validation to prevent arbitrary account insertion
    - Validate all user-provided inputs with appropriate bounds checking
    - Close unused accounts to reclaim rent and prevent dangling accounts (`#[account(close = recipient)]`)
    - Verify expected account relationships to prevent account confusion attacks

## On‑Chain Data Handling

- **`gill` and `@solana/kit` Integration:**
    - Implement optimized, non-blocking API calls to reduce latency
    - Use connection pooling and proper error handling for RPC interactions
    - Implement transaction retries with proper backoff strategies
    - Structure transactions to minimize fees by batching related operations
    - Use preflight checks to validate transactions before submission

- **Metaplex & NFT Integration:**
    - Follow Metaplex token standards for metadata creation and management
    - Implement proper verification of NFT ownership and metadata
    - Use compressed NFTs (cNFTs) for collections with many assets to reduce costs
    - Employ proper royalty enforcement when implementing marketplaces

## Performance and Optimization

- **Best Practices:**
    - Minimize transaction costs by bundling operations efficiently.
    - Exploit parallelism--don't serialize steps unnecessarily.
    - Regularly benchmark your code to spot and remove performance

- **Computational Efficiency:**
    - Optimize account sizes to minimize storage costs
    - Use zero-copy deserialization for large data structures
    - Minimize instruction count and computational complexity
    - Profile programs to identify compute unit bottlenecks
    - Consider native Rust implementation for performance-critical sections

- **Advanced Optimization Techniques:**
    - Use instruction introspection to optimize control flow
    - Implement caching strategies to reduce redundant computations
    - Consider specialized data structures for frequent operations
    - Use direct syscalls in performance-critical paths (with appropriate safety measures)

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
    - Program ID is defined in `Anchor.toml`
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
