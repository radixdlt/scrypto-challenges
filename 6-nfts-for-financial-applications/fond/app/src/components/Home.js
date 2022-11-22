export default function Home() {
    return (
        <div className="hero flex-1 bg-base-200 max-w-4xl">
            <div className="hero-content flex-col lg:flex-row gap-8">
                <img src={require("../data/hero.png")} className="max-w-sm shadow-2xl"/>
                <div>
                    <h1 className="text-5xl font-bold">Fraction</h1>
                    <h1 className="text-5xl font-bold">Investing made easy</h1>
                    <p className="py-6">With the help of fractional NFTs, taking ownership in rare assets
                    is now accessible for everyone</p>
                    <button className="btn btn-secondary">Get Started</button>
                </div>
            </div>
        </div>
    )
}