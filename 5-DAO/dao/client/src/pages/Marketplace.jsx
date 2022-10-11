import BuyTokens from '../components/BuyTokens';

const Marketplace = () => {
  return (
    <div>
      <h2 className="text-2xl font-bold mb-2">
        Welcome To The DAO Marketplace
      </h2>
      <p>
        Find new projects to support, contribute to, sell or trade member tokens
        with other community members.
      </p>
      <BuyTokens />
    </div>
  );
};

export default Marketplace;
