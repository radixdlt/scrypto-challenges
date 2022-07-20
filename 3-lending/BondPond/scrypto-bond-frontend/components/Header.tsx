
import Logo from "./Logo"
import LoginButton from './LoginButton'
import Nav from './Nav'
import Version from "./Version"
import Meta from "../Meta"

export default function Header() {
  return (
    <>
      <Meta /> 
      <header className='bg-black text-slate-200 font-khula pt-4 pb-2 w-full sticky top-0'>
        <div className='flex justify-between items-center'>
          <div className="flex items-center">
            <Logo />
            <Version /> 
          </div>
          <div className="flex pr-4 items-center">
            <Nav />
            <LoginButton />
          </div>
        
          
        </div>
      </header>
    </>
  )
}
