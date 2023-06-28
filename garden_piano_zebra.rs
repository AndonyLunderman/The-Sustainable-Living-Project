// Examples of sustainable living
//Sustainable living has become a popular way of life in recent years. People are turning away from traditional, wasteful practices and striving to make their lifestyles more environmentally friendly. This can include reducing resource consumption, recycling, and investing in renewable energy sources. Below are some examples of sustainable living that anyone can incorporate into their day-to-day lives.

    // Reduce energy consumption
    // Consider purchasing energy efficient appliances whenever possible. Turn off the lights and unplug electronics when not in use. Take advantage of natural light during the day and switch to LED light bulbs. Wash clothes in cold water and hang them to dry.

    // Reduce water consumption
    // Install low-flow shower heads and aerators on faucets. Fix any leaks quickly. Take shorter showers and limit the use of baths. Collect rainwater and use it for water your plants and garden. Use reclaimed water for irrigation.

    // Reduce waste
    // Recycle as much as possible. Compost kitchen scraps. Reuse items such as jars and bags whenever possible. Avoid purchasing items with excess packaging. Support local farmer’s markets and buy items in bulk to reduce the amount of packaging.

    // Sustainable transportation
    // Invest in a hybrid or electric vehicle. Carpool with friends and family. Take public transportation or bike to work. Utilize ride-sharing services whenever possible. Avoid excessive idling from your car.

    // Purchase sustainable products
    // Look for sustainable and organic products. Buy items made with recycled materials. Avoid single-use plastics. Purchase items with minimal packaging. Support eco-conscious companies.

    // Support renewable energy
    // Research renewable energy sources such as solar, wind, and geothermal. Invest in home solar panels. Purchase green energy from your energy provider.

    // Participate in community initiatives
    // Join a local environmental group to stay updated on upcoming events and initiatives. Join beach or park clean-ups. Plant trees in your neighborhood. Educate your friends and family on sustainable living.

mod sustainable_living {
    // Reduce energy consumption
    pub fn reduce_energy_consumption() {
        println!("Consider purchasing energy efficient appliances whenever possible. \
        Turn off the lights and unplug electronics when not in use. \
        Take advantage of natural light during the day and switch to LED light bulbs. \
        Wash clothes in cold water and hang them to dry.");
    }

    // Reduce water consumption
    pub fn reduce_water_consumption() {
        println!("Install low-flow shower heads and aerators on faucets. \
        Fix any leaks quickly. Take shorter showers and limit the use of baths. \
        Collect rainwater and use it for water your plants and garden. Use reclaimed water for irrigation.");
    }

    // Reduce waste
    pub fn reduce_waste() {
        println!("Recycle as much as possible. Compost kitchen scraps. \
        Reuse items such as jars and bags whenever possible. \
        Avoid purchasing items with excess packaging. \
        Support local farmer’s markets and buy items in bulk to reduce the amount of packaging.");
    }

    // Sustainable transportation
    pub fn sustainable_transportation() {
        println!("Invest in a hybrid or electric vehicle. \
        Carpool with friends and family. Take public transportation or bike to work. \
        Utilize ride-sharing services whenever possible. Avoid excessive idling from your car.");
    }

    // Purchase sustainable products
    pub fn sustainable_products() {
        println!("Look for sustainable and organic products. \
        Buy items made with recycled materials. Avoid single-use plastics. \
        Purchase items with minimal packaging. Support eco-conscious companies.");
    }

    // Support renewable energy
    pub fn support_renewable_energy() {
        println!("Research renewable energy sources such as solar, wind, and geothermal. \
        Invest in home solar panels. Purchase green energy from your energy provider.");
    }

    // Participate in community initiatives
    pub fn participate_in_initiatives() {
        println!("Join a local environmental group to stay updated on upcoming events and initiatives. \
        Join beach or park clean-ups. Plant trees in your neighborhood. \
        Educate your friends and family on sustainable living.");
    }
}

fn main() {
    sustainable_living::reduce_energy_consumption();
    sustainable_living::reduce_water_consumption();
    sustainable_living::reduce_waste();
    sustainable_living::sustainable_transportation();
    sustainable_living::sustainable_products();
    sustainable_living::support_renewable_energy();
    sustainable_living::participate_in_initiatives();
}