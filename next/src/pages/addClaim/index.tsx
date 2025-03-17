// pages/create-claim.tsx
import { useState, ChangeEvent, DragEvent, FormEvent } from 'react';
import Head from 'next/head';
import Link from 'next/link';
import { useRouter } from 'next/router';

interface FormData {
  latitude: string;
  longitude: string;
  startDate: string;
  tokensRequested: string;
  description: string;
  media: File[];
}

interface PreviewData {
  location: string;
  startDate: string;
  tokensRequested: string;
  status: string;
}

export default function CreateClaim(): JSX.Element {
  const router = useRouter();
  const [formData, setFormData] = useState<FormData>({
    latitude: '',
    longitude: '',
    startDate: '',
    tokensRequested: '',
    description: '',
    media: []
  });

  const [previewData, setPreviewData] = useState<PreviewData>({
    location: '12.9716° N, 77.5946° E',
    startDate: 'January 15, 2025',
    tokensRequested: '1,000 ZKC',
    status: 'Pending'
  });

  const [isDragging, setIsDragging] = useState<boolean>(false);

  const handleChange = (e: ChangeEvent<HTMLInputElement | HTMLTextAreaElement>): void => {
    const { name, value } = e.target;
    setFormData({
      ...formData,
      [name]: value
    });
  };

  const handleDragOver = (e: DragEvent<HTMLDivElement>): void => {
    e.preventDefault();
    setIsDragging(true);
  };

  const handleDragLeave = (e: DragEvent<HTMLDivElement>): void => {
    e.preventDefault();
    setIsDragging(false);
  };

  const handleDrop = (e: DragEvent<HTMLDivElement>): void => {
    e.preventDefault();
    setIsDragging(false);
    
    const files = Array.from(e.dataTransfer.files);
    
    // Handle file uploads
    if (files.length > 0) {
      // In a real app, you'd upload these files to a server
      // For this example, we'll just update the state
      setFormData({
        ...formData,
        media: [...formData.media, ...files]
      });
    }
  };

  const handleFileSelect = (e: ChangeEvent<HTMLInputElement>): void => {
    if (e.target.files) {
      const files = Array.from(e.target.files);
      
      if (files.length > 0) {
        setFormData({
          ...formData,
          media: [...formData.media, ...files]
        });
      }
    }
  };

  const handleSubmit = (e: FormEvent<HTMLFormElement>): void => {
    e.preventDefault();
    // Here you would process the submission, send data to API, etc.
    console.log("Submitting form data:", formData);
    
    // For this example, we'll just show an alert and keep the preview data
    alert("Claim submitted successfully!");
  };

  return (
    <div className="min-h-screen bg-gray-50">
      <Head>
        <title>Create New Carbon Credit Claim | ZK Carbon</title>
        <meta name="description" content="Create a new carbon credit claim for your tree planting project" />
        <link rel="icon" href="/favicon.ico" />
      </Head>



      {/* Main Content */}
      <main className="max-w-4xl mx-auto py-8 px-4">
        <h1 className="text-3xl font-bold text-center mb-8 text-gray-800">Create New Carbon Credit Claim</h1>

        {/* Form Section */}
        <form onSubmit={handleSubmit} className="bg-white rounded-lg shadow-sm p-6 mb-8">
          {/* Media Upload */}
          <div className="mb-6">
            <label className="block text-gray-700 mb-2">Upload Media</label>
            <div 
              className={`border-2 border-dashed rounded-lg p-8 text-center ${isDragging ? 'border-green-500 bg-green-50' : 'border-gray-300'}`}
              onDragOver={handleDragOver}
              onDragLeave={handleDragLeave}
              onDrop={handleDrop}
            >
              <div className="flex flex-col items-center">
                <svg 
                  className="w-12 h-12 text-gray-400 mb-4" 
                  fill="none" 
                  stroke="currentColor" 
                  viewBox="0 0 24 24" 
                  xmlns="http://www.w3.org/2000/svg"
                >
                  <path 
                    strokeLinecap="round" 
                    strokeLinejoin="round" 
                    strokeWidth="2" 
                    d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
                  />
                </svg>
                <p className="text-gray-500 mb-2">Drag and drop images or videos here, or click to browse</p>
                <input 
                  type="file" 
                  multiple 
                  className="hidden" 
                  id="file-upload" 
                  onChange={handleFileSelect} 
                />
                <label 
                  htmlFor="file-upload" 
                  className="cursor-pointer text-green-600 hover:text-green-700"
                >
                  Browse files
                </label>
              </div>
            </div>
            {formData.media.length > 0 && (
              <div className="mt-4 flex flex-wrap gap-2">
                {formData.media.map((file, index) => (
                  <div key={index} className="text-sm bg-gray-100 px-3 py-1 rounded-full">
                    {file.name}
                  </div>
                ))}
              </div>
            )}
          </div>

          {/* Location Fields */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
            <div>
              <label htmlFor="latitude" className="block text-gray-700 mb-2">Latitude</label>
              <input
                type="text"
                id="latitude"
                name="latitude"
                value={formData.latitude}
                onChange={handleChange}
                placeholder="Enter latitude"
                className="w-full p-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-green-500"
              />
            </div>
            <div>
              <label htmlFor="longitude" className="block text-gray-700 mb-2">Longitude</label>
              <input
                type="text"
                id="longitude"
                name="longitude"
                value={formData.longitude}
                onChange={handleChange}
                placeholder="Enter longitude"
                className="w-full p-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-green-500"
              />
            </div>
          </div>

          {/* Date and Tokens */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
            <div>
              <label htmlFor="startDate" className="block text-gray-700 mb-2">Start Date</label>
              <input
                type="date"
                id="startDate"
                name="startDate"
                value={formData.startDate}
                onChange={handleChange}
                className="w-full p-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-green-500"
              />
            </div>
            <div>
              <label htmlFor="tokensRequested" className="block text-gray-700 mb-2">Tokens Demanded</label>
              <input
                type="text"
                id="tokensRequested"
                name="tokensRequested"
                value={formData.tokensRequested}
                onChange={handleChange}
                placeholder="Enter amount"
                className="w-full p-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-green-500"
              />
            </div>
          </div>

          {/* Project Description */}
          <div className="mb-6">
            <label htmlFor="description" className="block text-gray-700 mb-2">Project Description</label>
            <textarea
              id="description"
              name="description"
              value={formData.description}
              onChange={handleChange}
              placeholder="Describe your tree planting project..."
              rows={5}
              className="w-full p-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-green-500"
            ></textarea>
          </div>

          {/* Submit Button */}
          <div className="flex justify-end">
            <button
              type="submit"
              className="px-6 py-2 bg-green-600 text-white rounded hover:bg-green-700 transition-colors"
            >
              Submit Claim
            </button>
          </div>
        </form>

        {/* Preview Section */}
        <div className="bg-white rounded-lg shadow-sm p-6">
          <h2 className="text-xl font-semibold mb-6 text-gray-800">Preview Your Claim</h2>
          
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <h3 className="text-gray-500 mb-2">Location</h3>
              <div className="flex items-center">
                <svg className="w-5 h-5 text-gray-600 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"></path>
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"></path>
                </svg>
                <p className="text-gray-800">{previewData.location}</p>
              </div>
            </div>
            
            <div>
              <h3 className="text-gray-500 mb-2">Start Date</h3>
              <div className="flex items-center">
                <svg className="w-5 h-5 text-gray-600 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path>
                </svg>
                <p className="text-gray-800">{previewData.startDate}</p>
              </div>
            </div>
            
            <div>
              <h3 className="text-gray-500 mb-2">Tokens Requested</h3>
              <div className="flex items-center">
                <svg className="w-5 h-5 text-gray-600 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                </svg>
                <p className="text-gray-800">{previewData.tokensRequested}</p>
              </div>
            </div>
            
            <div>
              <h3 className="text-gray-500 mb-2">Status</h3>
              <div className="flex items-center">
                <span className="px-3 py-1 bg-yellow-100 text-yellow-800 text-sm rounded-full">
                  {previewData.status}
                </span>
              </div>
            </div>
          </div>
        </div>
      </main>
    </div>
  );
}