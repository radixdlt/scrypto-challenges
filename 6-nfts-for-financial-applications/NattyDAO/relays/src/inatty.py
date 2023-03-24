# Connect to the iNaturalist API and get the data for a given user.
# There's a lot more we can do around metadata.
# (e.g. extra rarity for reviewed observations...)
import requests
import sys
import pandas as pd
import numpy as np
import os
import json
from dotenv import load_dotenv
load_dotenv()

# path needed for cronjob access
script_path: str = os.path.dirname(os.path.realpath(__file__))

################################
# a class for this iNatty module instatiated per user
class iNatty(object):

    def __init__(self, user_name):
        self.user_name = user_name
        self.observedNew = []
        self.observedTotal = {}

    def get_obs_from_db(self) -> dict:
        with open(os.path.join(script_path, 'observed.json'), 'r') as f:
            observed = json.loads(f.read())
            return observed
    
    def get_observations(self) -> None:
        # Get past observations for all users
        self.observedTotal = self.get_obs_from_db()

        # Get the observations for the user
        url = 'https://api.inaturalist.org/v1/observations'
        params = {'user_id': self.user_name, 'per_page': 25, 'order_by': 'observed_on', 'order': 'desc'}
        r = requests.get(url, params=params)

        newOnes = False
        for obs in r.json()['results']:
            if str(obs['id']) not in self.observedTotal:
                o = self.parse_observation(obs)
                if o:
                    self.observedNew.append(o)
                    self.observedTotal[str(obs['id'])] = o
                    newOnes = True
        
        if newOnes:
            self.save_obs_to_db()
            # if there's a new one, we call the Radix Transaction Manifest to mint the NFT

        return None

    def parse_observation(self, obs) -> dict:
        # Parse the observations for the user
        obs_parsed = None
        try:
            obs_parsed = {
                "id": str(obs['id']),
                "user_name": self.user_name,
                "link": "https://www.inaturalist.org/observations/" + str(obs['id']),
                "observed_on": obs["observed_on"],
                "species": obs["species_guess"],
                "imageUrlSmall": obs["photos"][0]["url"],
                "imageUrlLarge": obs["photos"][0]["url"].replace("square", "original")
            }
        except Exception as e:
            print('error parsing observation: ', e, json.dumps(obs, indent=4, sort_keys=True))
            pass
        return obs_parsed

    def get_user_info(self) -> dict:
        user_data = requests.get('https://api.inaturalist.org/v1/users/' + self.user_name + '?key=' + api_key).json()
        return user_data

    def save_obs_to_db(self) -> None:
        # write over the file with the new data
        with open(os.path.join(script_path, 'observed.json'), 'w') as f:
            f.write(json.dumps(self.observedTotal, indent=4, sort_keys=True))
        return None
            
################################
"""
Take a user name and return the observations for that user
Get username as first param from command line
"""
def main():
    user = sys.argv[1]
    c = iNatty(user)
    c.get_observations()
    return c.observedNew

################################
if __name__ == '__main__':
    main()