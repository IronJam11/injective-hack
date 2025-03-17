import React from 'react';


interface Organization {
  id: string;
  name: string;
  location: string;
  reputationScore: number;
  carbonCredits: number;
  imageUrl: string;
}

const OrganizationsPage: React.FC = () => {
  const organizations: Organization[] = [
    {
      id: '1',
      name: 'Green Energy Solutions',
      location: 'San Francisco, CA',
      reputationScore: 4.8,
      carbonCredits: 2500,
      imageUrl: '/green-energy-solutions.jpg'
    },
    {
      id: '2',
      name: 'Sustainable Farms Co.',
      location: 'Portland, OR',
      reputationScore: 4.8,
      carbonCredits: 1800,
      imageUrl: '/sustainable-farms.jpg'
    },
    {
      id: '3',
      name: 'Ocean Cleanup Tech',
      location: 'Seattle, WA',
      reputationScore: 4.9,
      carbonCredits: 3200,
      imageUrl: '/ocean-cleanup.jpg'
    },
    {
      id: '4',
      name: 'Solar Power Industries',
      location: 'Austin, TX',
      reputationScore: 4.7,
      carbonCredits: 2800,
      imageUrl: '/solar-power.jpg'
    }
  ];

  return (
    <div className="min-h-screen bg-gray-50">
     

      {/* Main content */}
      <main className="p-6 max-w-6xl mx-auto">
        <div className="mb-6">
          <h2 className="text-2xl font-bold text-gray-800">Organizations</h2>
          <p className="text-gray-600">Browse and support eco-friendly organizations</p>
        </div>

        {/* Organizations grid */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {organizations.map((org) => (
            <div key={org.id} className="bg-white rounded-lg shadow p-6">
              <div className="flex items-center mb-4">
                <div className="w-12 h-12 rounded-full bg-gray-200 overflow-hidden mr-4">
                  <img src={org.imageUrl} alt={org.name} className="w-full h-full object-cover" />
                </div>
                <div>
                  <h3 className="font-semibold text-gray-800">{org.name}</h3>
                  <p className="text-gray-500 text-sm">{org.location}</p>
                </div>
              </div>
              
              <div className="border-t border-gray-100 pt-4">
                <div className="flex justify-between items-center mb-2">
                  <span className="text-gray-600 text-sm">Reputation Score</span>
                  <span className="text-green-600 font-medium">{org.reputationScore}/5.0</span>
                </div>
                <div className="flex justify-between items-center mb-4">
                  <span className="text-gray-600 text-sm">Carbon Credits</span>
                  <span className="text-green-600 font-medium">{org.carbonCredits.toLocaleString()}</span>
                </div>
                <button className="w-full bg-green-600 hover:bg-green-700 text-white py-2 rounded transition-colors">
                  Lend Money
                </button>
              </div>
            </div>
          ))}
        </div>
      </main>

      {/* Footer */}
      <footer className="py-6 text-center text-gray-500 text-sm">
        © 2025 EcoLend. All rights reserved.
      </footer>
    </div>
  );
};

export default OrganizationsPage;