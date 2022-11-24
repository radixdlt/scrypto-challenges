import { CurrencyDollarIcon, PlusIcon } from '@heroicons/react/20/solid'
import {NavLink} from "react-router-dom"

export default function Collection() {
    return (
        <div className="relative block w-96 rounded-lg border-2 border-dashed border-neutral-content p-12 text-center">
            <div className="text-center">
                <CurrencyDollarIcon className="h-12 w-12 mx-auto" aria-hidden="true" />
                <h3 className="mt-2 text-sm font-medium text-neutral-content">No Investments</h3>
                <p className="mt-1 text-sm text-primary/50">Get started by investing in your first Asset</p>
                <div className="mt-6">
                    <NavLink to="/invest" className="btn btn-primary">
                        <PlusIcon className="-ml-1 mr-2 h-5 w-5" aria-hidden="true" />
                        Invest
                    </NavLink>
                </div>
            </div>
        </div>
    )
}