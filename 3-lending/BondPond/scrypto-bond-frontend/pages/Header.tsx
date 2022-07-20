
import Logo from "../components/Logo"
//import LoginButton from '../components/LoginButton'
import Nav from '../components/Nav'
import Version from "../components/Version"
//import Meta from "../components/Meta"

export default function Header() {
  return (
    <>

      <header className='bg-dark text-slate-200 font-khula pt-4 pb-2 w-full sticky top-0'>
        <div className='flex justify-between items-center'>
          <div className="flex items-center">
            <Logo />
            <Version /> 
          </div>
          
          
        </div>
      </header>
    </>
  )
}
