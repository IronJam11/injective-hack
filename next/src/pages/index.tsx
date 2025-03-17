// pages/index.js
import Head from 'next/head';
import Link from 'next/link';
import { useState } from 'react';
import ConnectWallet from '@/components/ConnectWallet';

export default function Home() {
  const [isLoggedIn, setIsLoggedIn] = useState(false);

  return (
    <div className="min-h-screen bg-white">
      <Head>
        <title>ZK Carbon - Decentralized Carbon Credit Marketplace</title>
        <meta name="description" content="Empowering organizations to contribute to a sustainable future through verifiable tree plantation and carbon credit trading." />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      {/* Hero Section */}
      <section className="py-12 px-6 md:px-12 md:py-16 bg-gray-50">
        <div className="max-w-7xl mx-auto grid grid-cols-1 md:grid-cols-2 gap-12 items-center">
          <div>
            <h1 className="text-4xl md:text-5xl font-bold text-gray-900 leading-tight">
              Decentralized Carbon Credit Marketplace
            </h1>
            <p className="mt-6 text-lg text-gray-600">
              Empowering organizations to contribute to a sustainable future through verifiable tree plantation and carbon credit trading.
            </p>
            <div className="mt-8">
              <button className="px-6 py-3 bg-green-600 text-white rounded-md hover:bg-green-700 transition-colors">
                Start Trading
              </button>
            </div>
          </div>
          <div className="relative">
            <img 
              src="/img.png" 
              alt="Green globe with trees and blockchain nodes" 
              className="rounded-lg shadow-xl w-full"
            />
          </div>
        </div>
      </section>

      {/* Stats Section */}
      <section className="py-16 px-6">
        <div className="max-w-7xl mx-auto grid grid-cols-1 md:grid-cols-3 gap-8">
          <div className="bg-white p-8 rounded-lg border border-gray-100 shadow-sm text-center">
            <h2 className="text-4xl font-bold text-green-600">1.2M</h2>
            <p className="mt-2 text-gray-600">Carbon Credits Minted</p>
          </div>
          <div className="bg-white p-8 rounded-lg border border-gray-100 shadow-sm text-center">
            <h2 className="text-4xl font-bold text-green-600">450+</h2>
            <p className="mt-2 text-gray-600">Active Organizations</p>
          </div>
          <div className="bg-white p-8 rounded-lg border border-gray-100 shadow-sm text-center">
            <h2 className="text-4xl font-bold text-green-600">2.5M</h2>
            <p className="mt-2 text-gray-600">Trees Planted</p>
          </div>
        </div>
      </section>

      {/* How It Works Section */}
      <section className="py-16 px-6 bg-gray-50">
        <div className="max-w-7xl mx-auto">
          <h2 className="text-3xl font-bold text-center text-gray-900 mb-16">How It Works</h2>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-12">
            <div className="text-center">
              <div className="mx-auto bg-green-100 w-20 h-20 rounded-full flex items-center justify-center mb-6">
                <svg className="h-8 w-8 text-green-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                </svg>
              </div>
              <h3 className="text-xl font-semibold mb-4">Plant Trees</h3>
              <p className="text-gray-600">Organizations plant trees and document their environmental impact</p>
            </div>
            <div className="text-center">
              <div className="mx-auto bg-green-100 w-20 h-20 rounded-full flex items-center justify-center mb-6">
                <svg className="h-8 w-8 text-green-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
              </div>
              <h3 className="text-xl font-semibold mb-4">Claim Credits</h3>
              <p className="text-gray-600">Submit proofs and receive verified carbon credits</p>
            </div>
            <div className="text-center">
              <div className="mx-auto bg-green-100 w-20 h-20 rounded-full flex items-center justify-center mb-6">
                <svg className="h-8 w-8 text-green-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4" />
                </svg>
              </div>
              <h3 className="text-xl font-semibold mb-4">Trade & Borrow</h3>
              <p className="text-gray-600">Trade or lend carbon credits in our marketplace</p>
            </div>
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="py-16 px-6 bg-green-600 text-white">
        <div className="max-w-4xl mx-auto text-center">
          <h2 className="text-3xl font-bold mb-6">Ready to Make an Impact?</h2>
          <p className="text-lg mb-8">
            Join our growing network of environmentally conscious organizations and start your journey towards carbon neutrality.
          </p>
          <button className="px-8 py-3 bg-white text-green-600 rounded-md hover:bg-gray-100 transition-colors font-medium">
            Create Account
          </button>
        </div>
      </section>

      {/* Footer */}
      <footer className="bg-gray-800 text-white py-12 px-6">
        <div className="max-w-7xl mx-auto grid grid-cols-1 md:grid-cols-4 gap-8">
          <div>
            <h3 className="text-xl font-bold mb-4">ZK Carbon</h3>
            <p className="text-gray-300">
              Empowering a sustainable future through blockchain technology.
            </p>
          </div>
          <div>
            <h4 className="font-semibold mb-4">Marketplace</h4>
            <ul className="space-y-2">
              <li><Link href="/browse" className="text-gray-300 hover:text-white">Browse Credits</Link></li>
              <li><Link href="/create-claim" className="text-gray-300 hover:text-white">Create a Claim</Link></li>
              <li><Link href="/verification" className="text-gray-300 hover:text-white">Verification Process</Link></li>
            </ul>
          </div>
          <div>
            <h4 className="font-semibold mb-4">Resources</h4>
            <ul className="space-y-2">
              <li><Link href="/about" className="text-gray-300 hover:text-white">About Us</Link></li>
              <li><Link href="/docs" className="text-gray-300 hover:text-white">Documentation</Link></li>
              <li><Link href="/faq" className="text-gray-300 hover:text-white">FAQ</Link></li>
            </ul>
          </div>
          <div>
            <h4 className="font-semibold mb-4">Connect</h4>
            <ul className="space-y-2">
              <li><Link href="/contact" className="text-gray-300 hover:text-white">Contact Us</Link></li>
              <li><Link href="https://twitter.com/zkcarbon" className="text-gray-300 hover:text-white">Twitter</Link></li>
              <li><Link href="https://discord.gg/zkcarbon" className="text-gray-300 hover:text-white">Discord</Link></li>
            </ul>
          </div>
        </div>
        <div className="max-w-7xl mx-auto mt-12 pt-6 border-t border-gray-700 text-center text-gray-400">
          <p>&copy; {new Date().getFullYear()} ZK Carbon. All rights reserved.</p>
        </div>
      </footer>
    </div>
  );
}