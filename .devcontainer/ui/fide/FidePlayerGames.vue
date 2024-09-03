<template>
    <div>
      <!-- The main content of the FIDE player games page -->
  
      <!-- Button to download games in PGN format -->
      <button @click="downloadGames" class="button">
        Download Games
      </button>
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent } from 'vue';
  import axios from 'axios';
  
  export default defineComponent({
    name: 'FidePlayerGames',
    
    // Props that the component accepts
    props: {
      // The ID of the player whose games are being displayed
      playerId: {
        type: String,
        required: true,
      },
    },
  
    methods: {
      /**
       * Method to download games for the specified player in PGN format.
       * This method makes an API request to the Lichess backend to fetch the games
       * and then triggers a download in the user's browser.
       */
      async downloadGames() {
        try {
          // Make a GET request to the API to fetch the player's games in blob format
          const response = await axios.get(`/api/fide/${this.playerId}/games`, {
            responseType: 'blob', // Important to set response type as 'blob' for binary data
          });
  
          // Create a Blob from the response data, specifying the correct MIME type
          const blob = new Blob([response.data], { type: 'application/x-chess-pgn' });
  
          // Create a URL for the blob
          const url = window.URL.createObjectURL(blob);
  
          // Create a temporary <a> element to trigger the download
          const link = document.createElement('a');
          link.href = url;
          link.setAttribute('download', `${this.playerId}_games.pgn`); // Set the download filename
  
          // Append the link to the document body and trigger the click
          document.body.appendChild(link);
          link.click();
  
          // Clean up by removing the link from the document
          link.remove();
        } catch (error) {
          // Log any errors to the console for debugging purposes
          console.error('Error downloading games:', error);
        }
      },
    },
  });
  </script>
  
  <style scoped>
  /* Styles for the download button */
  .button {
    margin-top: 10px;
    padding: 10px 20px;
    background-color: #4CAF50; /* Green background */
    color: white;              /* White text */
    border: none;
    cursor: pointer;           /* Pointer cursor on hover */
  }
  
  .button:hover {
    background-color: #45a049; /* Darker green on hover */
  }
  </style>
  
  <template>
    <div>
      <!-- The main content of the FIDE player games page -->
  
      <!-- Button to share the link to the player's games -->
      <button @click="shareGames" class="button">
        Share Games
      </button>
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent } from 'vue';
  
  export default defineComponent({
    name: 'FidePlayerGames',
  
    // Props that the component accepts
    props: {
      // The ID of the player whose games are being displayed
      playerId: {
        type: String,
        required: true,
      },
    },
  
    methods: {
      /**
       * Method to share the URL to the player's games.
       * This method uses the Web Share API if available, or falls back to copying
       * the link to the clipboard if the Web Share API is not supported.
       */
      shareGames() {
        // Construct the URL to the player's games on the Lichess site
        const url = `${window.location.origin}/fide/${this.playerId}`;
  
        // Check if the Web Share API is available in the user's browser
        if (navigator.share) {
          navigator.share({
            title: `Games of ${this.playerId}`, // Title of the share dialog
            url: url,                          // The URL to be shared
          }).catch((error) => {
            // Handle any errors that occur during the sharing process
            console.error('Error sharing link:', error);
          });
        } else {
          // Fallback: Copy the URL to the clipboard if Web Share API is not supported
          navigator.clipboard.writeText(url).then(() => {
            // Notify the user that the link has been copied
            alert('Link copied to clipboard!');
          }, (error) => {
            // Handle any errors that occur during the copy process
            console.error('Error copying link:', error);
          });
        }
      },
    },
  });
  </script>
  
  <style scoped>
  /* Styles for the share button, same as the download button */
  .button {
    margin-top: 10px;
    padding: 10px 20px;
    background-color: #4CAF50; /* Green background */
    color: white;              /* White text */
    border: none;
    cursor: pointer;           /* Pointer cursor on hover */
  }
  
  .button:hover {
    background-color: #45a049; /* Darker green on hover */
  }
  </style>
  