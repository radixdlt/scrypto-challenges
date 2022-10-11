import CreateDAO from '../components/CreateDAO';
import CreateBallot from '../components/CreateBallot';
import CreateProposal from '../components/CreateProposal';

const Dashboard = () => {
  return (
    <div>
      <h2 className="text-2xl font-bold mb-2">Dashboard</h2>
      <div className="border-2">
        <CreateDAO />
      </div>
      <div className="border-2 mt-8">
        <CreateBallot />
      </div>
      <div className="border-2 mt-8">
        <CreateProposal />
      </div>
    </div>
  );
};

export default Dashboard;
