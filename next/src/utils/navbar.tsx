import ConnectWallet from '@/components/ConnectWallet';
import Head from 'next/head';
import Link from 'next/link';


const Navbar = () => {
    return (
        <nav className="bg-white border-b border-gray-100 py-4 px-6 md:px-12">
                <div className="flex justify-between items-center">
                <Link href="/">
                    <span className="text-2xl font-bold text-green-600 cursor-pointer">ZK Carbon</span>
                </Link>
                <div className="flex items-center space-x-8">
                    <Link href="/claims" className="text-gray-600 hover:text-green-600">Claims</Link>
                    <Link href="/organizations" className="text-gray-600 hover:text-green-600">Organizations</Link>
                    <Link href="/create-claim" className="text-gray-600 hover:text-green-600">Create Claim</Link>
                    <div className="flex items-center space-x-2">
                    <div className="h-3 w-3 bg-green-500 rounded-full"></div>
                    <ConnectWallet />
                    </div>
                </div>
            </div>
        </nav>
    );
};
export default Navbar;


